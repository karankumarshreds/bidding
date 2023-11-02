use tokio;
#[allow(unused)]
use tower::ServiceExt;
use sqlx::{Connection, PgConnection};
use auth::models::user::{LoginPayload, LoginResponse, User, SignupPayload};
use auth::run;
use auth::configuration;

#[tokio::test]
async fn sign_in() {
    let port = spawn_app();// it runs concurrently with the execution
    let response = login(port).await;
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
    let port = spawn_app();
    let response = login(port).await;
    let token_response = response.json::<LoginResponse>().await.expect("cannot parse token from response");
    let client = reqwest::Client::new();
    let user = client
        .get(format!("http://localhost:{}/who-am-i", port))
        .header("Authorization", token_response.token)
        .send()
        .await
        .expect("Failed to execute request")
        .json::<User>()
        .await
        .unwrap();
    assert_eq!(user.id, "1");
}

#[tokio::test]
async fn signup() {
    let port = spawn_app();
    let client = reqwest::Client::new();
    let payload = SignupPayload{
            username: String::from("user_a"),
            password: String::from("password"),
        };
    let connection_string = configuration::get_configuration()
        .expect("Cannot load config")
        .database
        .connection_string();
    let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to PG");
    /*
    let response = client
        .post(format!("http://localhost:{}/signup", port))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), 200);
    let token_response = response.json::<LoginResponse>()
        .await
        .expect("cannot parse token from signup response");
    assert!(token_response.token.len() > 10);
    */
    let user = sqlx::query!("select * from users where username=$1", payload.username)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch user");
    assert_eq!(user.password, payload.password);
    sqlx::query!("delete from users where username=$1", payload.username)
        .execute(&mut connection)
        .await
        .expect("Failed to cleanup");
}

// This makes sure the test suite is totally decoupled 
// from the code base. In future if we decide to re-write
// the application in another language, we could still use 
// the same test suite to trigger the application.
// Example: bash command to launch some other language server
fn spawn_app() -> u16 {
    // We use the port 0 because Port 0 is the special port in OS.
    // Trying to bind with port 0, OS will scan for an available port automatically
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(run(listener));
    port
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
