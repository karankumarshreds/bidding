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
use routes::{sign_up,login,who_am_i};
use models::user::{AppState,JWTSettings};
use middlewares::auth::with_auth;
use sqlx::PgPool;

pub async fn run(tcp_listener: TcpListener) -> std::io::Result<()> {
    let config = configuration::get_configuration().unwrap();
    let db_connection_url = config.database.connection_string();
    let db_pool = PgPool::connect(&db_connection_url).await.unwrap();
    let shared_state = Arc::new(
        AppState {
            db_connection: db_pool,
            jwt: JWTSettings {
                secret: config.jwt.secret,
                expiration: config.jwt.expiration,
            }
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
