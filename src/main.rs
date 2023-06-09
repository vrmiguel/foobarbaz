/// Our HTTP API
mod api;
/// Our error enum and Result alias
mod error;
/// Database interactions
mod repository;
/// Consumes tasks off of the database and sends them over to the
/// [Worker](crate::worker::Worker).
mod task_consumer;
/// The actual worker that finishes tasks
mod worker;

pub use error::Result;
use sea_orm::{ConnectOptions, Database};
use task_consumer::TaskConsumer;

#[tokio::main]
async fn main() -> crate::Result<()> {
    let pg_url = std::env::var("PG_URL").unwrap_or_else(|_| {
        println!("PG_URL not set. Assuming default");
        "postgresql://postgres@localhost:5555/postgres".into()
    });

    let conn = Database::connect(connection_options(pg_url)).await?;
    // Start tracing
    tracing_subscriber::fmt().compact().init();

    // Start our HTTP API
    let server = api::spawn_server(conn.clone())?;

    let (_, sender) = worker::Worker::spawn();

    TaskConsumer::spawn(sender, conn);

    // Wait for the HTTP server to close
    server.await?;

    Ok(())
}

fn connection_options(url: String) -> ConnectOptions {
    ConnectOptions::new(url).sqlx_logging(false).to_owned()
}
