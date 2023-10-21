// mod routes;
// mod models;
// mod middlewares;
//
use auth::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}

