use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Activating environment logging.
    // We are calling just info-level logs if the RUST_LOG env variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Read configuration file
    let configuration = get_configuration().expect("Failed to read application configuration file");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    // Connection to Postgres DB through a connection pool
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to get connection to Postgres DB");

    // let port = listener.local_addr().unwrap().port();
    run(listener, connection_pool)?.await

    // Launch server as a background task thanks to tokio::spawn
}
