use std::net::TcpListener;
use subscription_add::{startup::run, configuration};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener, connection_pool)?.await
}
