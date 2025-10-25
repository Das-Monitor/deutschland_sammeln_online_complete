mod database;

#[macro_use] extern crate rocket;

use chrono::NaiveDateTime;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::database::{establish_connection, get_gemeinde_namen_from_database, validate_session_token, AuthenticationError, GemeindeDaten};

#[derive(Serialize, Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[derive(Responder)]
enum LoginFailureResponse<T>{
    Unauthorized(status::Unauthorized<T>),
    Custom(Custom<T>)
}

#[post("/database/login", format="application/json", data="<data>")]
async fn login(data: Json<UserCredentials>)
    -> Result<status::Accepted<String>, LoginFailureResponse<&'static str>> {
    println!("processing request");
    match establish_connection(&data.username, &data.password).await{
        Ok(token) => Ok(status::Accepted(token)),
        Err(e) => Err(match e{
            AuthenticationError::Unauthorized(e) => {
                LoginFailureResponse::Unauthorized(e)
            }
            AuthenticationError::Internal(_) => {
                LoginFailureResponse::Custom(
                    Custom(Status::InternalServerError,
                        "Something went wrong internally, please try again")
                )
            }
        })
    }
}

#[derive(Serialize, Deserialize)]
struct GemeindeDatenRequestData {
    username: String,
    token: String,
    occasion: String,
    starting_timestamp: Option<NaiveDateTime>
}

#[derive(Responder)]
enum GemeindeDatenRequestFailureResponse<T> {
    Unauthorized(status::Unauthorized<T>),
    Custom(Custom<T>)
}

impl <T> From<Custom<T>> for GemeindeDatenRequestFailureResponse<T> {
    fn from(value: Custom<T>) -> Self {
        GemeindeDatenRequestFailureResponse::Custom(value)
    }
}

#[post("/database/gemeinden", format="json", data = "<data>")]
async fn sende_gemeinde_daten(data: Json<GemeindeDatenRequestData>)
    -> Result<status::Accepted<Json<GemeindeDaten>>,
        GemeindeDatenRequestFailureResponse<String>>
{
    if !validate_session_token(&data.username, &data.token).await.unwrap_or(false) {
        return Err(
            GemeindeDatenRequestFailureResponse::Unauthorized(
                status::Unauthorized("Invalid session token. Please login again.".to_string()))
        )
    }

    Ok(
        status::Accepted(
            Json(get_gemeinde_namen_from_database(
                &data.occasion,
                data.starting_timestamp.unwrap_or(NaiveDateTime::default())
            ).await
                .map_err(
                    |e| {
                        println!("{:?}" ,e);
                        Custom(
                        Status::InternalServerError,
                        format!("Something went wrong internally, please try again.")
                    )}
                )?
            )
        )
    )
}

#[launch]
async fn rocket() -> _ {
    database::init_db().await;

    rocket::build().mount("/", routes![sende_gemeinde_daten])
        .mount("/", routes![login])
}