use serde::Serialize;
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use std::sync::Arc;
use axum::extract::{
    State,
    Json,
};
use std::collections::HashMap;
use axum::http::StatusCode;

pub struct AppState {
    pub users_set: HashMap<String, User>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}

impl User {
    pub fn new() -> HashMap<String, User> {
        return HashMap::from([
            (
                String::from("1"), 
                User {
                    id: "1".to_string(),
                    username: "user1".to_string(),
                    password: "password".to_string(),
                }
            ),
        ])
    }
}

#[derive(Serialize)]
struct Claims {
    user_id: String,
    username: String,
}

const JWT_KEY: &'static str =  "asdf";

fn create_token(user_id: &str, username: &str, jwt_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        user_id: user_id.to_string(),
        username: username.to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(jwt_key.as_bytes());
    Ok(encode(&header, &claims, &key)?)
}

pub async fn login_handle(
        State(state): State<Arc<AppState>>, 
        Json(loginPayload): Json<LoginPayload>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
    let users_set = &state.users_set;

    if let Some(user) = users_set.get("1").cloned() {
        if user.password != loginPayload.password {
            return Err(StatusCode::UNAUTHORIZED)
        };
        let token = create_token(&user.id, &user.username, JWT_KEY)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(LoginResponse{ token }));
    } 
    return Err(StatusCode::UNAUTHORIZED)
}

