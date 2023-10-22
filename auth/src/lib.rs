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
// use tokio::net::TcpListener;
use std::net::TcpListener;
use std::net::SocketAddr;
use routes::{login_handle, who_am_i};
use models::user::{AppState, User};
use middlewares::auth::with_auth;


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
    let config = configuration::get_configuration().unwrap();
    println!("{}", config.database.connection_string());
    check_envs().unwrap();

    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
        }
    );
                                  
    let app = Router::new()
        .route("/who-am-i", get(who_am_i))
        .route_layer(from_fn(with_auth))
        // without auth
        .route("/login", post(login_handle))
        .with_state(shared_state);
    Server::from_tcp(tcp_listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
