use crate::startup;
use actix_web::{HttpResponse, Responder};
use sqlx::Connection;
use std::net::TcpListener;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // "reqwest" allows us to create a client and perform requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", address))
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration =
        crate::configuration::get_configuration().expect("Failed to get configuration from file");
    let connection_string = configuration.database.connection_string();

    let mut connection = sqlx::PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    // Act
    let body = "name=fulmicotone98&email=fulmicotone98%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "fulmicotone98@gmail.com");
    assert_eq!(saved.name, "fulmicotone98");
}

/* Table-driven test / Parametrized test -> The same assertion is run against a set of known invalid input bodies that we expect to fail */
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=fulmicotone98", "missing email"),
        ("email=fulmicotone98%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, err_msg) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content_Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert!(
            response.status().is_client_error(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            err_msg
        )
    }
}

// Launch our application in the background ~somehow~
#[allow(dead_code)]
fn spawn_app() -> String {
    // bind port 0 will trigger an OS scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to bind address");

    // Launch server as a background task thanks to tokio::spawn
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
