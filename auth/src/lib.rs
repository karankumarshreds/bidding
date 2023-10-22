pub mod configuration;
pub mod routes;
pub mod models;
pub mod middlewares;

use std::sync::Arc;
use axum::{
    Router,
    Server,
    routing::{get, post},
    middleware::from_fn,
};
use std::net::TcpListener;
use routes::{login, sign_up, who_am_i};
use models::user::{AppState, User};
use middlewares::auth::with_auth;
use sqlx::{Connection, PgConnection};


fn check_envs() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let vars = vec!["JWT_KEY"];
    for v in vars {
        dotenv::dotenv().ok();
        std::env::var(v)?;
    }
    Ok(())
}

pub async fn run(tcp_listener: TcpListener) -> std::io::Result<()> {
    check_envs().unwrap();
    let config = configuration::get_configuration().unwrap();
    let db_connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
            db_connection,
        }
    );
                                  
    let app = Router::new()
        // with auth
        .route("/who-am-i", get(who_am_i))
        .route_layer(from_fn(with_auth))
        // without auth
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .with_state(shared_state);
    Server::from_tcp(tcp_listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
