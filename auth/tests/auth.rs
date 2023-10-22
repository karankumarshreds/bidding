use tokio;
#[allow(unused)]
use tower::ServiceExt;
use auth::models::user::{LoginPayload, LoginResponse, User};
use auth::run;

#[tokio::test]
async fn sign_up() {
    //let (handle, port) = spawn_app(); // the task is spawned as toon as you await it
                       // and it runs concurrently with the execution
    let port = spawn_app();

    let response = login(port).await;
    println!("response.status(): {}", response.status());
    assert_eq!(response.status(), 200);
    assert_ne!(response.content_length(), Some(0));
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
