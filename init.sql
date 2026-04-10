-- init.sql

-- Enable the TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create the standard PostgreSQL table
CREATE TABLE IF NOT EXISTS price_history (
    timestamp TIMESTAMPTZ NOT NULL,
    market_id VARCHAR NOT NULL,
    outcome VARCHAR NOT NULL,
    price FLOAT NOT NULL,
    size FLOAT NOT NULL
);

-- Convert the standard table into a TimescaleDB hypertable
-- Parititioning by the 'timestamp' column
SELECT create_hypertable('price_history', 'timestamp');

-- Create a composite index on timestamp and market_id
-- (A descending index on timestamp often provides better performance for time-series queries)
CREATE INDEX IF NOT EXISTS idx_price_history_time_market
    ON price_history (timestamp DESC, market_id);
