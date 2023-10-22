use auth::models::user::LoginPayload;
use tokio;
use auth::run;

#[tokio::test]
async fn sign_up() {
    spawn_app().await; // the task is spawned as toon as you await it
                       // and it runs concurrently with the execution
                       // of this function

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
    println!("response.status(): {}", response.status());
    assert_eq!(response.status(), 200);
    assert_ne!(response.content_length(), Some(0));
}

async fn spawn_app() -> tokio::task::JoinHandle<Result<(), std::io::Error>> {
    let handle = tokio::spawn(run());
    handle
}

// This makes sure the test suite is totally decoupled 
// from the code base. In future if we decide to re-write
// the application in another language, we could still use 
// the same test suite to trigger the application.
// Example: bash command to launch some other language server
async fn _spawn_app() -> Result<(), std::io::Error> {
    let handle =  tokio::spawn(run());
    let _ =  handle.await.map_err(|e| std::io::Error::from(e))?;
    Ok(())
}
