import os
from contextlib import asynccontextmanager
from typing import Dict, Any

from fastapi import FastAPI, HTTPException
import asyncpg
import pandas as pd

# Load DB URL from environment or fallback to phase 1 defaults
DATABASE_URL = os.getenv(
    "DATABASE_URL", 
    "postgres://chronos_admin:secure_password_here@localhost:5432/chronos"
)

# We use asynccontextmanager to manage lifespan events in FastAPI (recommended standard)
@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup logic
    print("Initializing asyncpg connection pool...")
    app.state.pool = await asyncpg.create_pool(dsn=DATABASE_URL, min_size=1, max_size=5)
    yield
    # Shutdown logic
    print("Closing connection pool...")
    if app.state.pool:
        await app.state.pool.close()

app = FastAPI(title="Chronos Analytics API", lifespan=lifespan)

@app.get("/api/markets/{market_id}/trends")
async def get_market_trends(market_id: str) -> Dict[str, Any]:
    """
    Query the price_history table for the last 24 hours of data for a specific market.
    Loads data into a Pandas DataFrame to calculate a 1-hour moving average for the odds,
    grouped by market outcome.
    """
    pool = app.state.pool
    if not pool:
        raise HTTPException(status_code=500, detail="Database connection pool unavailable.")

    # High-performance TimescaleDB query fetching exactly the last 24H of ticks
    query = """
        SELECT timestamp, outcome, price, size
        FROM price_history
        WHERE market_id = $1 AND timestamp >= NOW() - INTERVAL '24 hours'
        ORDER BY timestamp ASC
    """

    async with pool.acquire() as connection:
        records = await connection.fetch(query, market_id)

    if not records:
        return {"market_id": market_id, "trends": {}}

    # Convert to standard Python dictionaries, then into a Pandas DataFrame
    data = [dict(record) for record in records]
    df = pd.DataFrame(data)

    # Ensure timestamp is datetime and set as index for the rolling calculation
    df['timestamp'] = pd.to_datetime(df['timestamp'])
    df.set_index('timestamp', inplace=True)

    result_trends = {}

    # Polymarket markets have multiple outcomes (e.g. Yes/No). We must calculate the MA independently.
    for outcome, group_df in df.groupby('outcome'):
        # Calculate 1-hour moving average logic. 
        # C-backed Pandas .rolling is highly efficient for ARM64 and heavily optimized.
        group_df['price_ma_1h'] = group_df['price'].rolling('1h').mean()
        
        # Replace NaN (the first few ticks before a full hour accumulates) with None for JSON mapping
        # or we could fill it with the raw price using `.fillna`
        group_df['price_ma_1h'] = group_df['price_ma_1h'].fillna(group_df['price'])
        
        # Reset the index to expose timestamp back as a regular dictionary column
        group_df.reset_index(inplace=True)
        
        # Convert timestamp objects to ISO strings for straightforward serialization
        group_df['timestamp'] = group_df['timestamp'].dt.strftime('%Y-%m-%dT%H:%M:%S.%fZ')
        
        # Orient records safely converts dataframe rows to dict lists
        result_trends[outcome] = group_df.to_dict(orient="records")

    return {
        "market_id": market_id,
        "trends": result_trends
    }
