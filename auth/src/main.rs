use auth::run;
use auth::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = configuration::get_configuration().unwrap().application_port;
    run(port).await
}

