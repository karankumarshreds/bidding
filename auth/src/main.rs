mod handlers;
mod models;
mod middlewares;

use std::sync::Arc;
use std::env::var as env_var;
use axum::{
    Router,
    Server,
    routing::{get, post},
    middleware::from_fn,
};

use std::net::SocketAddr;
use handlers::auth::{login_handle, who_am_i};
use models::user::{AppState, User};
use middlewares::auth::with_auth;
use dotenv;

fn check_envs() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let vars = vec!["JWT_KEY"];
    for v in vars {
        dotenv::dotenv().ok();
        std::env::var(v)?;
    }
    Ok(())
}
#[tokio::main]
async fn main() {
    check_envs().unwrap();

    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
        }
    );
                                  
    let app = Router::new()
        .route("/who-am-i", get(who_am_i)) // with auth
        .route("/new-bid", post())
        .route_layer(from_fn(with_auth))

        .route("/login", post(login_handle)) // without auth
        .with_state(shared_state);

    let addr = SocketAddr::from(([127,0,0,1], 8000));

    Server::binds(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
