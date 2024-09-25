//! tests/health_check.rs

use std::net::TcpListener;

use email_newsletter::configuration::{get_configuration, DatabaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_checker", &app.address))
        .send()
        .await
        .expect("Failed to get response");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
    // arrange
    let app = spawn_app().await;
    let app_address = &app.address;
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");
    let client = reqwest::Client::new();

    // Act
    let body = "name=karan%20kk4&email=karan%40rust.com";
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to get response");

    // Assert
    assert_eq!(200, response.status().as_u16());

    // check db
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "karan@rust.com");
    assert_eq!(saved.name, "karan kk4");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let app_address = app.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "missing both name and email"),
        ("name=karan", "missing the email"),
        ("email=karan@admin.com", "missing the name"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let url = format!("{}/subscriptions", &app_address);

        let response = client
            .post(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to get response");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with 400 error message when the payload was {}",
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to get configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_databse(&configuration.database).await;

    let server = email_newsletter::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_databse(config: &DatabaseSettings)-> PgPool{
    // Create Database
    let mut connection =  
    PgConnection::connect(&config.connection_string_without_db())
    .await
    .expect("Failed to connect to postgres");

    connection.execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
    .await
    .expect("Failed to create database.");

    // Migrate
    let connection_pool =  PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");
connection_pool

}
