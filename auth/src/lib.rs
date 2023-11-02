pub mod configuration;
pub mod routes;
pub mod models;
pub mod middlewares;

use std::sync::{Arc,Mutex};
use axum::{
    Router,
    Server,
    routing::{get, post},
    middleware::from_fn,
};
use std::net::TcpListener;
use routes::{login, who_am_i, sign_up};
use models::user::{AppState, User, JWTSettings};
use middlewares::auth::with_auth;
use sqlx::{Connection, PgConnection};

pub async fn run(tcp_listener: TcpListener) -> std::io::Result<()> {
    let config = configuration::get_configuration().unwrap();
    let db_connection = Arc::new(Mutex::new(PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database")));
    let shared_state = Arc::new(
        AppState {
            users_set: User::new(),
            db_connection,
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
