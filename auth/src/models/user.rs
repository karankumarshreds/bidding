use std::collections::HashMap;

pub struct AppState {
    pub users_set: HashMap<String, User>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
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


