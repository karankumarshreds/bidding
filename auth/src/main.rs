use auth::run;
use auth::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = configuration::get_configuration().unwrap().application_port;
    let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    run(listener).await
}

