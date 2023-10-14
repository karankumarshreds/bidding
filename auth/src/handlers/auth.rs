use std::error::Error;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation,
};
use std::sync::Arc;
use axum::extract::{
    State,
    Json,
};
use axum::http::StatusCode;
use crate::models::user::AppState;

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    user_id: String,
    username: String,
    exp: usize,
}

pub const JWT_KEY: &'static str =  "asdf";

fn create_token(user_id: &str, username: &str, jwt_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        user_id: user_id.to_string(),
        username: username.to_string(),
        exp: 10000000000,
    };
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(jwt_key.as_bytes());
    Ok(encode(&header, &claims, &key)?)
}

pub fn validate_token(token: &str, jwt_key: &str) -> Result<Claims, Box<dyn Error>> {
    let payload = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_key.as_bytes()),
            &Validation::new(Algorithm::HS256)
        )?.claims;
    return Ok(payload);
}

pub async fn login_handle(
        State(state): State<Arc<AppState>>, 
        Json(login_payload): Json<LoginPayload>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
    let users_set = &state.users_set;

    if let Some(user) = users_set.get("1").cloned() {
        if user.password != login_payload.password {
            return Err(StatusCode::UNAUTHORIZED)
        };
        let token = create_token(&user.id, &user.username, JWT_KEY)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(LoginResponse{ token }));
    } 
    return Err(StatusCode::UNAUTHORIZED)
}

pub async fn test_handler() -> Result<(), StatusCode> {
    Ok(())
}
