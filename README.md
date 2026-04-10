# Chronos: High-Performance Polymarket Ingestion & Analytics

Chronos is a high-performance data pipeline designed to track real-time trade events from the Polymarket CLOB (Central Limit Order Book). It is optimized to run on Edge hardware (like the **NVIDIA Jetson Orin Nano**) using external NVMe storage powered by **TimescaleDB**.

## 🚀 Architecture

The system consists of three main components core to high-frequency data handling:

1.  **Ingestion Service (Rust):** An asynchronous service built with `tokio` and `tokio-tungstenite` that connects to the Polymarket WebSocket. it listens for `price_change` or `last_trade_price` events and persists them instantly to the database.
2.  **Database (TimescaleDB/PostgreSQL):** Runs in Docker using the TimescaleDB extension for time-series optimization. Data is stored in a `hypertable` for ultra-fast time-based queries.
3.  **Analytics API (Python/FastAPI):** A lightweight web server providing endpoints for calculating technical indicators, such as **1-hour Rolling Moving Averages**, leveraging `pandas` for efficient ARM64-optimized processing.

## 🛠️ Installation

### Prerequisites
*   **Docker** & **Docker Compose**
*   **Rust Toolchain** (cargo)
*   **Python 3.10+**
*   SSH access to your Jetson device (if deploying remotely).

### Quick Start

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/arhimonde/Chronos.git
    cd Chronos
    ```

2.  **Configuration:**
    Open `ingestion/src/websocket.rs` and update the `assets_ids` variable with the Polymarket Market IDs you wish to track.

3.  **Launch Project:**
    The `RUN_PROJECT.sh` script automates database startup, Python virtual environment setup, and Rust service compilation:
    ```bash
    chmod +x RUN_PROJECT.sh
    ./RUN_PROJECT.sh
    ```

## 📈 Usage

### Monitoring Ingestion
Once started, you will see real-time logs in your terminal:
`[INFO] Saved Trade -> Market: 0x... | Outcome: ... | Price: $...`

### Accessing the Analytics API
The API runs on port `8000`. You can access the interactive Swagger documentation here:
👉 `http://<JETSON_IP>:8000/docs`

Use the `/api/markets/{market_id}/trends` endpoint to retrieve price history and calculated moving averages.

## 🗄️ Database Schema
The `price_history` table includes:
*   `timestamp`: Exact event time (UTC).
*   `market_id`: The smart contract address of the market.
*   `outcome`: The specific outcome (e.g., "Yes", "No").
*   `price`: The odds/price at that moment.
*   `size`: The trade volume.

## 🔧 Troubleshooting
*   **WebSocket Disconnects:** Polymarket frequently resets idle connections. The Rust service includes **auto-reconnect** logic (retries every 5 seconds).
*   **No Data in API:** Ensure the chosen Market IDs are actively being traded on Polymarket. Check the terminal logs for "Saved Trade" messages.

---
Developed for predictive monitoring and algorithmic trading research.
