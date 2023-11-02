To integrate connection pooling in your project, you can use the r2d2 crate along with sqlx. Here's a basic example of how you can set up connection pooling:

Add r2d2 and r2d2_sqlx to your Cargo.toml:
toml
Copy code
[dependencies]
sqlx = "0.5"
r2d2 = "0.9"
r2d2_sqlx = "0.17"
Update your main.rs or lib.rs:
rust
Copy code
use axum::{
    extract::Extension,
    handler::{get, Handler},
    response::Html,
    Router,
};
use r2d2::Pool;
use r2d2_sqlx::SqlxConnectionManager;
use std::sync::Arc;

#[derive(Debug)]
struct User {
    // Your user struct definition
}

#[derive(Debug)]
struct JWTSettings {
    // Your JWT settings struct definition
}

#[derive(Debug)]
struct AppState {
    users_set: User,
    db_pool: Pool<SqlxConnectionManager<tokio_postgres::Postgres>>,
    jwt: JWTSettings,
}

async fn who_am_i(Extension(state): Extension<Arc<AppState>>) -> Html<&'static str> {
    // Access state and perform some operation
    Html("Hello from who_am_i handler!")
}

// Other handlers and routes...

#[tokio::main]
async fn main() {
    // Configure your database connection string
    let database_url = "your_database_url_here";

    // Create a connection manager
    let manager = SqlxConnectionManager::<tokio_postgres::Postgres>::new(database_url);

    // Create a connection pool
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool");

    // Create your shared state
    let shared_state = Arc::new(AppState {
        users_set: User, // Initialize your user set
        db_pool: pool,
        jwt: JWTSettings, // Initialize your JWT settings
    });

    // Create your Axum app
    let app = Router::new()
        .route("/who-am-i", get(who_am_i))
        .with_state(shared_state);

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

In this example, I replaced PgConnection with Pool<SqlxConnectionManager<tokio_postgres::Postgres>> in the AppState, and I used the r2d2 pool to manage the database connections. The who_am_i handler still receives the shared state as an extension.

This setup allows multiple threads to borrow connections from the pool concurrently, and you don't need to use a Mutex to lock the entire connection. The r2d2_sqlx crate integrates seamlessly with sqlx, making it easy to use connection pooling with your Axum application. Adjust the code according to your specific needs and database configuration.
