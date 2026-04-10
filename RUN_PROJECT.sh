#!/bin/bash

# Project Chronos - Master Launch Script (v2)
# Target: NVIDIA Jetson Orin Nano (Ubuntu ARM64)

echo "--- Phase 1: TimescaleDB ---"
if ! docker compose up -d; then
    echo "Retrying with sudo..."
    sudo docker compose up -d
fi

echo "--- Phase 2: Python Analytics API ---"
cd analytics_api
if [ ! -d "venv" ]; then
    python3 -m venv venv
fi
source venv/bin/activate
pip install -r requirements.txt
# Start API in background and redirect logs to a file
nohup uvicorn main:app --host 0.0.0.0 --port 8000 > api.log 2>&1 &
ANALYTICS_PID=$!
echo "Analytics API started in background (PID: $ANALYTICS_PID)"
cd ..

echo "--- Phase 3: Rust Ingestion Service ---"
cd ingestion

# Ensure Cargo is available
if ! command -v cargo &> /dev/null; then
    echo "ERROR: 'cargo' not found. Please install Rust by running:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then run: source \$HOME/.cargo/env"
    exit 1
fi

export DATABASE_URL=postgres://chronos_admin:secure_password_here@localhost:5432/chronos
export RUST_LOG=info
cargo run --release

# Clean up background process on exit
trap "kill $ANALYTICS_PID" EXIT
