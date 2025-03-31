use std::net::TcpListener;

#[tokio::main]
async fn main() {
    // bind port 0 will trigger an OS scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    // Launch server as a background task thanks to tokio::spawn
    tokio::spawn(server);

    // format!("http://127.0.0.1:{}", port)
}
