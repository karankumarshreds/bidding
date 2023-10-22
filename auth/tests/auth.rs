use auth::models::user::{LoginPayload, LoginResponse, User};
use tokio;
use auth::run;

#[tokio::test]
async fn sign_up() {
    spawn_app().await; // the task is spawned as toon as you await it
                       // and it runs concurrently with the execution
                       // of this function
    let response = login().await;
    println!("response.status(): {}", response.status());
    assert_eq!(response.status(), 200);
    assert_ne!(response.content_length(), Some(0));
}

#[tokio::test]
async fn who_am_i() {
    spawn_app().await;
    let response = login().await;
    let token_response = response.json::<LoginResponse>().await.expect("cannot parse token from response");
    let client = reqwest::Client::new();
    let user = client
        .get("http://localhost:8000/who-am-i")
        .header("Authorization", token_response.token)
        .send()
        .await
        .expect("Failed to execute request")
        .json::<User>()
        .await
        .unwrap();
    assert_eq!(user.id, "2");
}


// This makes sure the test suite is totally decoupled 
// from the code base. In future if we decide to re-write
// the application in another language, we could still use 
// the same test suite to trigger the application.
// Example: bash command to launch some other language server
async fn spawn_app() -> tokio::task::JoinHandle<Result<(), std::io::Error>> {
    // We use the port 0 because Port 0 is the special port in OS.
    // Trying to bind with port 0, OS will scan for an available port automatically
    let handle = tokio::spawn(run(0));
    handle
}

async fn login() -> reqwest::Response {
    let client = reqwest::Client::new();
    let payload = LoginPayload{
            username: String::from("user1"),
            password: String::from("password"),
        };
    let response = client
        .post("http://localhost:8000/login")
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    response
}
