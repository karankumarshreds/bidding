use std::error::Error;
use serde::{Serialize, Deserialize};
// use sqlx::{PgPool, Arguments, Connection, Pool, Postgres};
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation,
};
use std::sync::Arc;
// use axum::extract::{State, Json, Extension};
use axum::extract::{State, Json};
use axum::Extension;
use axum::http::StatusCode;
use crate::models::user::{AppState, LoginPayload, LoginResponse, SignupPayload, WhoAmIResponse};


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub exp: usize,
}

fn create_token(user_id: &str, username: &str, jwt_key: &str, _exp: i32) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        user_id: user_id.to_string(),
        username: username.to_string(),
        exp: 10000000000,
    };
    println!("using claims {:#?}", claims);
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

pub async fn sign_up(
        State(state): State<Arc<AppState>>,
        Json(signup_payload): Json<SignupPayload>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
    // check if the user with same email is there
    let conn = &state.db_connection;
    let user = sqlx::query!("select * from users where username=$1", signup_payload.username)
        .fetch_optional(conn)
        .await
        .map_err(|err| {
            println!("ERROR: Unable to execute the query {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    if let Some(_) = user {
        println!("the user is already in use");
        return Err(StatusCode::BAD_REQUEST);
    } 

    println!("creating a new user");
    sqlx::query!("insert into users(username, password) values($1, $2)", signup_payload.username, signup_payload.password)
        .execute(conn)
        .await
        .map_err(|err| {
            println!("ERROR: failed to create new user for: {}\n{}",signup_payload.username, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let user = sqlx::query!("select id, username from users where username=$1", signup_payload.username)
        .fetch_one(conn)
        .await
        .map_err(|err| {
            println!("ERROR: failed to fetch user: {} \n {:#?}", signup_payload.username, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let jwt_cfg = &state.jwt;
    let token = create_token(&user.id.to_string(), &user.username, &jwt_cfg.secret, jwt_cfg.expiration)
        .map_err(|err| {
            println!("ERROR: failed to create token {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    return Ok(Json(LoginResponse{token}));
}

pub async fn login(
        State(state): State<Arc<AppState>>, 
        Json(login_payload): Json<LoginPayload>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
    let conn = &state.db_connection;
    let user = sqlx::query!(
            "select id, username from users where username=$1 and password=$2",
            login_payload.username,
            login_payload.password,
        )
        .fetch_optional(conn)
        .await
        .map_err(|err| {
            println!("ERROR: unable to fetch user from db: {:#?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = create_token(
        &user.id.to_string(),
        &user.username,
        &state.jwt.secret,
        state.jwt.expiration,
    ).map_err(|err| {
        println!("ERROR: failed to create token {:#?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    return Ok(Json(LoginResponse{token}));
}

pub async fn who_am_i(
        State(_state): State<Arc<AppState>>,
        Extension(claims): Extension<Arc<Claims>>,
    ) -> Result<Json<WhoAmIResponse>, StatusCode> {
    let id = claims.user_id.parse::<i32>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let username = claims.username.clone();
    return Ok(Json(WhoAmIResponse{id, username}));
}
