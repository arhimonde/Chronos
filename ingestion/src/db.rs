use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::models::MarketData;

/// Establishes a database connection pool to the PostgreSQL instance.
pub async fn establish_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Executes an efficient SQL INSERT statement to save the tick data 
/// into the TimescaleDB 'price_history' hypertable.
/// Uses 'sqlx::query' to enforce prepared statements for safety & performance,
/// while avoiding compile-time DB checks so you can build without the DB running.
pub async fn save_market_data(pool: &PgPool, data: &MarketData) -> Result<(), sqlx::Error> {
    let query = r#"
        INSERT INTO price_history (timestamp, market_id, outcome, price, size)
        VALUES ($1, $2, $3, $4, $5)
    "#;

    sqlx::query(query)
        .bind(data.timestamp)
        .bind(&data.market_id)
        .bind(&data.outcome)
        .bind(data.price)
        .bind(data.size)
        .execute(pool)
        .await?;

    Ok(())
}
