use auth::models::user::LoginPayload;

#[tokio::test]
async fn sign_up() {
    spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let payload = LoginPayload{
            username: String::from(""),
            password: String::from(""),
        };
    let response = client
        .post("http://localhost:8000/api/login")
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_ne!(response.content_length(), Some(0));
}

// This makes sure the test suite is totally decoupled 
// from the code base. In future if we decide to re-write
// the application in another language, we could still use 
// the same test suite to trigger the application.
// Example: bash command to launch some other language server
async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
