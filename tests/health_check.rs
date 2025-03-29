#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // "reqwest" allows us to create a client and perform requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // test response of health_check API covers the full range of properties we are interested to check:
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    // the health check always returns a 200;
    // the health check’s response has no body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch server as a background task thanks to tokio::spawn
    tokio::spawn(server);
}
