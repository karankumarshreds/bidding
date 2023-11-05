use std::error::Error;
use axum::http::StatusCode;
use sqlx::{PgPool, Arguments, Connection, Pool, Postgres};
use sqlx::pool::PoolConnection;

// #[derive(Clone)]
pub struct AppState {
    pub db_connection: PgPool,
    pub jwt: JWTSettings,
}

impl AppState {
    pub async fn connect_db(&self) -> Result<PoolConnection<Postgres>, StatusCode> {
        self.db_connection.acquire().await.map_err(|err| {
            println!("ERROR: unable to acquire connection from pool: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }
}

pub struct JWTSettings {
        pub secret: String,
        pub expiration: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SignupPayload {
    pub username: String,
    pub password: String,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WhoAmIResponse {
    pub username: String,
    pub id: i32,
}
