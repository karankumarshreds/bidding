use tokio;
use sqlx::{Connection, Executor, PgConnection, PgPool};

use auth::models::user::{LoginPayload,LoginResponse,SignupPayload,WhoAmIResponse};
use auth::run;
use auth::configuration;

#[tokio::test]
async fn sign_in() {
    let app = spawn_app().await;// it runs concurrently with the execution
    let response = login(app.port).await;
    println!("response.status(): {}", response.status());
    assert_eq!(response.status(), 200);
    assert_ne!(response.content_length(), Some(0));
}

#[tokio::test]
async fn test_db() {
    let config = configuration::get_configuration().expect("cannot get db config");
    let connection = PgConnection::connect(&config.database.connection_string()).await;
    assert!(connection.is_ok());
}

#[tokio::test]
async fn who_am_i() {
    let app = spawn_app().await;
    let response = login(app.port).await;
    let token_response = response.json::<LoginResponse>().await.expect("cannot parse token from response");
    let client = reqwest::Client::new();
    let user = client
        .get(format!("http://localhost:{}/who-am-i", app.port))
        .header("Authorization", token_response.token)
        .send()
        .await
        .expect("Failed to execute request")
        .json::<WhoAmIResponse>()
        .await
        .unwrap();
    assert_eq!(user.id, 2);
}

#[tokio::test]
async fn signup() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let payload = SignupPayload{
            username: String::from("user_a"),
            password: String::from("password"),
        };
    let response = client
        .post(format!("http://localhost:{}/signup", app.port))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), 200);
    let token_response = response.json::<LoginResponse>()
        .await
        .expect("cannot parse token from signup response");
    assert!(token_response.token.len() > 10);
    let user = sqlx::query!("select * from users where username=$1", payload.username)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch user");
    assert_eq!(user.password, payload.password);
    sqlx::query!("delete from users where username=$1", payload.username)
        .execute(&app.db_pool)
        .await
        .expect("Failed to cleanup");
}

struct TestApp {
    port: u16,
    db_pool: sqlx::PgPool,
}
// This makes sure the test suite is totally decoupled 
// from the code base. In future if we decide to re-write
// the application in another language, we could still use 
// the same test suite to trigger the application.
// Example: bash command to launch some other language server
async fn spawn_app() -> TestApp {
    // We use the port 0 because Port 0 is the special port in OS.
    // Trying to bind with port 0, OS will scan for an available port automatically
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let db_pool = configure_test_db().await;

    tokio::spawn(run(listener));
    TestApp {
        port,
        db_pool
    }
}

async fn configure_test_db() -> sqlx::PgPool {
    let mut db_config = configuration::get_configuration().expect("failed to get config").database;
    db_config.db_name = uuid::Uuid::new_v4().to_string();

    // create a database
    let mut connection = PgConnection::connect(&db_config.connection_string_without_db_name())
        .await
        .expect("failed to connect to db");
    connection.execute(format!(r#"create database "{}""#, db_config.db_name).as_str())
        .await
        .expect("failed to create database");

    // run migrations
    let pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");
    pool
}

async fn login(port: u16) -> reqwest::Response {
    let client = reqwest::Client::new();
    let payload = LoginPayload{
            username: String::from("user1"),
            password: String::from("password"),
        };
    let response = client
        .post(format!("http://localhost:{}/login", port))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    response
}
