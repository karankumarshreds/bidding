mod handlers;
mod models;

use std::sync::Arc;
use axum::{
    Router,
    Server,
    routing::post
};

use std::net::SocketAddr;
use handlers::auth:: login_handle;
use models::user::{AppState, User};


#[tokio::main]
async fn main() {
    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
        }
    );
                                  
    let app = Router::new()
        .route("/login", post(login_handle))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127,0,0,1], 8000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
