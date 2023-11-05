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
// use axum::extract::{State, Json, Extension};
use axum::extract::{State, Json};
use axum::Extension;
use axum::http::StatusCode;
use crate::models::user::{AppState, LoginPayload, LoginResponse, SignupPayload};
use crate::configuration::get_configuration;


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
    let mut connection = state.db_connection.acquire()
        .await
        .map_err(|err| {
            eprintln!("Unable to connect to pool: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;
    let user = sqlx::query!("select * from users where username=$1", signup_payload.username)
        .fetch_optional(&mut connection)
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
        .execute(&mut connection)
        .await
        .map_err(|err| {
            println!("ERROR: failed to create new user for: {}\n{}",signup_payload.username, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let user = sqlx::query!("select id, username from users where username=$1", signup_payload.username)
        .fetch_one(&mut connection)
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
    let mut conn = state.db_connection.acquire()
        .await
        .map_err(|err| {
            eprintln!("Unable to connect to pool: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;
    let user = sqlx::query!(
            "select id, username from users where username=$1 and password=$2",
            login_payload.username,
            login_payload.password,
        )
        .fetch_optional(&mut conn)
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

/*
pub async fn who_am_i(
        State(state): State<Arc<AppState>>,
        Extension(claims): Extension<Arc<Claims>>,
    ) -> Result<Json<User>, StatusCode> {
    println!("got the claims {:#?}", claims);
    let users_set = &state.users_set;
    let user = users_set.get(&claims.user_id)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    return Ok(Json(user.clone()));
}
*/
