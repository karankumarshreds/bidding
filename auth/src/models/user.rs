use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use sqlx::PgConnection;

// #[derive(Clone)]
pub struct AppState {
    pub users_set: HashMap<String, User>,
    pub db_connection: Arc<Mutex<PgConnection>>,
    pub jwt: JWTSettings,
}

pub struct JWTSettings {
        pub secret: String,
        pub expiration: i32,
}

type UserId = String;
type UsersSet = HashMap<UserId, User>;

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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new() -> UsersSet {
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
    pub fn get_user_by_username<'a>(username: &'a str, users_set: &'a UsersSet) -> Option<&'a Self> {
        for (_, user) in users_set.iter() {
            if user.username == username  {
                return Some(user)
            }
        }
        return None
    }
}


