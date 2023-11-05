use sqlx::{PgPool, Arguments, Connection, Pool, Postgres};

// #[derive(Clone)]
pub struct AppState {
    pub db_connection: PgPool,
    pub jwt: JWTSettings,
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
