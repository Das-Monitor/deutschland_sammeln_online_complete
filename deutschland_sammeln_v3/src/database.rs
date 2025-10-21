use std::path::Path;
use std::string::ToString;
#[cfg(feature = "ssr")]
use leptos::server_fn::ServerFnError;
#[cfg(feature = "ssr")]
use std::sync::{Arc, OnceLock};
use chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use sqlx::{query};
#[cfg(feature = "ssr")]
use sqlx::{Executor, Row};
#[cfg(feature = "ssr")]
use sqlx::Acquire;
#[cfg(feature = "ssr")]
use tokio::io::AsyncWriteExt;

#[cfg(feature = "ssr")]
const DEFAULT_CONFIG: &str = r#"database_user = "user"
database_password = "password"
domain = "127.0.0.1"
port = "5432"
database_name = "deutschland_sammeln"
occasion = "default_config""#;

#[cfg(feature = "ssr")]
pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();
#[cfg(feature = "ssr")]
pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[cfg(feature = "ssr")]
#[derive(Deserialize, Debug)]
pub struct Config {
    database_user: String,
    database_password: String,
    domain: String,
    port: String,
    database_name: String,
    occasion: String,
}

#[cfg(feature = "ssr")]
struct GemeindeEintrag<'a> {
    gemeinde: &'a str,
    occasion: &'a str,
}

#[cfg(feature = "ssr")]
pub async fn init_db() {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut config_string: String = String::new();
    if let Ok(mut file) = File::open("./db_config.toml").await {
        file.read_to_string(&mut config_string)
            .await.expect("Failed to read the config file");
    }
    else {
        if !Path::new("/configs/db_config.toml").exists() {
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
    println!("Trying to connect to database under {}", &url);
    DB_POOL.set(PgPool::connect(&url).await.unwrap()).unwrap();

    CONFIG.set(config).unwrap();
}

#[cfg(feature = "ssr")]
pub async fn enter_name_into_database(name: Arc<str>) -> Result<bool, ServerFnError> {
    use std::ops::DerefMut;
    use sqlx::{query, Executor};

    let eintrag = GemeindeEintrag {
        gemeinde: &name.clone(),
        occasion: &CONFIG.get().unwrap().occasion,
    };

    let mut connection = DB_POOL.get().unwrap().acquire().await?;
    let mut transaction = connection.begin().await?;

    let duplicate = match query!("SELECT gemeinde_name FROM gemeinden WHERE gemeinde_name=$1 AND occasion=$2",
        &eintrag.gemeinde,
        &eintrag.occasion
    ).fetch_one(transaction.deref_mut()).await {
        Ok(_) => { true }
        Err(sqlx::Error::RowNotFound) => { false }
        Err(e) => {
            dbg!(e);
            panic!()
        }
    };

    if !duplicate {
        let _ = &transaction.execute(
            sqlx::query!(
                "INSERT INTO gemeinden (gemeinde_name, occasion, entry_timestamp) \
                    VALUES ($1, $2, $3)",
                &eintrag.gemeinde,
                &eintrag.occasion,
                &chrono::Local::now().naive_local()
            )
        ).await?;
    }

let _ = &transaction.execute(
        sqlx::query!(
            "INSERT INTO gemeinde_entry (gemeinde_name, occasion, entry_time) \
                VALUES ($1, $2, $3)",
            &eintrag.gemeinde,
            &eintrag.occasion,
            &chrono::Local::now().naive_local()
        )
    ).await?;

    transaction.commit().await?;

    Ok(duplicate)
}

#[cfg(feature = "ssr")]
pub async fn get_unique_entries_from_database(start_date: Option<NaiveDateTime>) -> Result<Vec<String>, ServerFnError> {
    let mut connection = DB_POOL.get().unwrap().acquire().await?;
    
    let entry_list: Vec<String> = connection
        .fetch_all(
            query!(
                "SELECT gemeinde_name FROM gemeinden WHERE occasion = $1 \
                AND entry_timestamp >= $2",
                CONFIG.get().unwrap().occasion, start_date
            )
        ).await?.iter().map(|item| item.get("gemeinde_name")).collect();
    
    Ok(entry_list)
}