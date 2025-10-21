use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::path::Path;
use argon2::{Argon2, PasswordHash};
use argon2::password_hash::{Encoding};
use chrono::{Local, NaiveDate, NaiveDateTime, TimeDelta};
use rocket::response::status::Unauthorized;
use rocket::serde::Deserialize;
use rocket::tokio::sync::OnceCell;
use serde::Serialize;
use sqlx::{query, Acquire, Executor, PgPool, Row};
use tokio::io::{AsyncWriteExt};

const DEFAULT_CONFIG: &str = r#"database_user = "user"
database_password = "password"
domain = "127.0.0.1"
port = "5432"
database_name = "deutschland_sammeln""#;

pub(crate) static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

#[derive(Deserialize, Debug)]
pub struct Config {
    database_user: String,
    database_password: String,
    domain: String,
    port: String,
    database_name: String,
}

pub async fn init_db() {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut config_string: String = String::new();
    if let Ok(mut file) = File::open("./db_config.toml").await {
        file.read_to_string(&mut config_string)
            .await.expect("Failed to read the config file");
    }
    else {
        if !Path::new("./db_config.toml").exists() {
            config_string = DEFAULT_CONFIG.to_string();
            println!("No config file \"db_config.toml\" could be found, deploying default config.");
            let mut file = File::create("./db_config.toml")
                .await
                .expect("Something went terribly wrong whilst creating default config file");
            file.write_all(DEFAULT_CONFIG.as_bytes())
                .await.expect("Failed to write to config file");
        }
        else {
            panic!("Failed to open config File \"db_config.toml\".\
            PLease fix the file or delete it, to deploy a new default config")
        }
    }
    let config: Config = toml::from_str(&config_string)
        .expect("Failed to parse the config file. Please fix, or delete to deploy new default file");
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &config.database_user,
        &config.database_password,
        &config.domain,
        &config.port,
        &config.database_name
    );
    DB_POOL.set(PgPool::connect(&url).await.unwrap()).unwrap();
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GemeindeDaten {
    gemeinde_namen: Vec<String>,
    last_timestamp: Option<NaiveDateTime>
}

pub(crate) async fn get_gemeinde_namen_from_database(starting_time: NaiveDateTime)
    -> Result<GemeindeDaten, Box<dyn Error>>
{
    let mut connection = DB_POOL.get().unwrap().acquire().await?;

    let (gemeinde_namen, time_stamps): (Vec<String>, Vec<NaiveDateTime>) = connection.fetch_all(
        query!(
            "SELECT gemeinde_name, entry_timestamp FROM gemeinden \
            WHERE entry_timestamp > $1 ORDER BY gemeinde_name", starting_time
            )
        ).await?
        .into_iter()
        .map(|r| (r.get::<String, &'static str>("gemeinde_name"),
            r.get::<NaiveDateTime, &'static str>("entry_timestamp")
        )
        ).collect();

    let last_timestamp = time_stamps.into_iter().max();

    Ok(GemeindeDaten{gemeinde_namen, last_timestamp})
}

#[derive(Debug)]
pub(crate) enum AuthenticationError{
    Unauthorized(Unauthorized<&'static str>),
    Internal(Box<dyn Error>)
}

impl From<Unauthorized<&'static str>> for AuthenticationError {
    fn from(value: Unauthorized<&'static str>) -> Self {
        AuthenticationError::Unauthorized(value)
    }
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self))
    }
}

impl Error for AuthenticationError{}

impl From<sqlx::Error> for AuthenticationError{
    fn from(value: sqlx::Error) -> Self {
        Self::Internal(Box::new(value))
    }
}

/// Validates the credentials, and returns a session token for the current session if the
/// credentials are valid, or an Error, if the credentials are invalid
pub(crate) async fn establish_connection(username: &str, password: &str)
    -> Result<String, AuthenticationError>
{
    let mut connection = DB_POOL.get().unwrap().acquire().await?;
    let internal_password_hash = connection.fetch_one(
        query!(
            "SELECT password_hash \
                FROM user_credentials \
                WHERE username = $1",
            username
        )
    ).await.map(|r| r.get::<String, &'static str>("password_hash"))
        .map_err(|_| AuthenticationError::Unauthorized(
            Unauthorized("Fehler: Dieser Nutzer existiert nicht")
        ))?;

    let password_hash =
        PasswordHash::parse(&internal_password_hash, Encoding::default())
            .expect("Failed to parse a password from the database");

    if password_hash.verify_password(&[&Argon2::default()], password).is_err(){
        return Err(AuthenticationError::Unauthorized(Unauthorized("Invalid user credentials")))
    };

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    const TOKEN_LEN: usize = 30;

    let token: String = (0..TOKEN_LEN)
        .map(|_| {
            let idx = rand::random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let mut transaction = connection.begin().await?;

    transaction.execute(
        query!(
            "INSERT INTO user_sessions(username, session_token, creation_timestamp) \
            VALUES($1, $2, $3)",
            &username, &token, Local::now().naive_local()
        )
    ).await?;

    transaction.commit().await?;

    Ok(token)
}

pub(crate) async fn validate_session_token(username: &str, token: &str) -> Result<bool, Box<dyn Error>> {
    let connection = DB_POOL.get().unwrap();
    Ok(match connection.fetch_one(
        query!(
            "SELECT creation_timestamp, session_token FROM user_sessions \
            WHERE username = $1 and session_token = $2 \
            ORDER BY creation_timestamp DESC", username, token
        )
    ).await {
        Ok(r) => {
            r.get::<NaiveDateTime, &'static str>("creation_timestamp").add(TimeDelta::days(1))
                >= Local::now().naive_local() &&
                r.get::<String, &'static str>("session_token") == token
        }
        Err(_) => {false}
    })
}