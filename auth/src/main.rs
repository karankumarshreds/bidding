mod handlers;
mod models;
mod middlewares;

use std::sync::Arc;
use axum::{
    Router,
    Server,
    routing::{get, post},
    middleware::from_fn,
};

use std::net::SocketAddr;
use handlers::auth::{login_handle, test_handler};
use models::user::{AppState, User};
use middlewares::auth::with_auth;


#[tokio::main]
async fn main() {
    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
        }
    );
                                  
    let app = Router::new()
        .route("/login", post(login_handle))
        .route("/test", get(test_handler))
        .route_layer(from_fn(with_auth))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127,0,0,1], 8000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
