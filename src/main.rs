use email_newsletter::startup::run;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use email_newsletter::configuration::get_configuration;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to get configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind express");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await.expect("Failed to connect to database");
    run(listener, connection_pool)?.await
}

/* 
 * Did you know you can embed your migrations in your application binary?
On startup, after creating your database connection or pool, add:

sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.
*/