use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{DBSettings, get_configuration};
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DBSettings) -> PgPool {
    //Create Database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    //Migrate Database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

// Launch our application in the background ~somehow~
#[allow(dead_code)]
async fn spawn_app() -> TestApp {
    // Get DB configuration
    let mut configuration =
        get_configuration().expect("Failed to read application configuration file");

    // Since we want TEST ISOLATION, a new fresh DB is needed everytime a test is run
    // We nee to create a new DB with random name
    configuration.database.db_name = Uuid::new_v4().to_string();

    // Connection to Postgres DB through PgPool
    let connection_pool = configure_database(&configuration.database).await;

    // Bind port 0 will trigger an OS scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");

    // Launch server as a background task thanks to tokio::spawn
    tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // "reqwest" allows us to create a client and perform requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=fulmicotone98&email=fulmicotone98%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "fulmicotone98@gmail.com");
    assert_eq!(saved.name, "fulmicotone98");
}

/* Table-driven test / Parametrized test -> The same assertion is run against a set of known invalid input bodies that we expect to fail */
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=fulmicotone98", "missing email"),
        ("email=fulmicotone98%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, err_msg) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", app.address))
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
