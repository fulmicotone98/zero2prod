use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read configuration file
    let configuration = get_configuration().expect("Failed to read application configuration file");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    // let port = listener.local_addr().unwrap().port();
    run(listener)?.await

    // Launch server as a background task thanks to tokio::spawn
}
