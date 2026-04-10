pub mod db;
pub mod models;
pub mod websocket;

use db::establish_connection;
use websocket::run_websocket_listener;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize env_logger with info-level default output
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Chronos Ingestion Service starting...");
    
    // Connect to database (falling back to our docker-compose defaults if DATABASE_URL is not set directly)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://chronos_admin:secure_password_here@localhost:5432/chronos".to_string());
    
    info!("Connecting to TimescaleDB...");
    let pool = match establish_connection(&database_url).await {
        Ok(p) => {
            info!("TimescaleDB connected successfully!");
            p
        },
        Err(e) => {
            error!("Fatal timescaleDB connection error: {}", e);
            return Err(e.into());
        }
    };

    info!("Initializing WebSocket loop...");
    // Start the WebSocket listener loop blockingly for this tokio process
    run_websocket_listener(pool).await;

    Ok(())
}
