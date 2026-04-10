use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use log::{info, error, warn};
use chrono::{DateTime, Utc};

use crate::db::save_market_data;
use crate::models::{MarketData, ws_message, PolymarketEvent};

// Correct Polymarket CLOB Market Channel URL
const WS_URL: &str = "wss://ws-subscriptions-clob.polymarket.com/ws/market";

pub async fn run_websocket_listener(pool: PgPool) {
    loop {
        info!("Connecting to Polymarket WebSocket at {}...", WS_URL);

        match connect_async(WS_URL).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Polymarket WebSocket!");
                let (mut write, mut read) = ws_stream.split();

                // Subscription logic
                // Using a common high-volume market ID as an example
                // Example: Trump Winning the US Election tokens
                let subscribe_msg = json!({
                    "type": "market",
                    // Active Microstrategy Token
                    "assets_ids": ["93592949212798121127213117304912625505836768562433217537850469496310204567695"],
                    "initial_dump": true,
                    "level": 2
                });

                if let Err(e) = write
                    .send(Message::Text(subscribe_msg.to_string()))
                    .await
                {
                    error!("Failed to send subscribe message: {}", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }

                info!("Subscription message sent. Monitoring trade events...");

                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(Message::Text(text)) => {
                            match serde_json::from_str::<ws_message>(&text) {
                                Ok(msg) => {
                                    let events = match msg {
                                        ws_message::Events(evs) => evs,
                                        ws_message::SingleEvent(ev) => vec![ev],
                                    };

                                    for ev in events {
                                        // We want rapid test data! We'll capture 'price_change' which fires continuously.
                                        if ev.event_type == "price_change" {
                                            if let (Some(price), Some(size)) = (ev.price, ev.size) {
                                                let timestamp = ev.timestamp.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now());
                                                
                                                let data = MarketData {
                                                    market_id: ev.market,
                                                    outcome: ev.outcome,
                                                    price,
                                                    size,
                                                    timestamp,
                                                };

                                                match save_market_data(&pool, &data).await {
                                                    Ok(_) => {
                                                        info!(
                                                            "Saved Trade -> Market: {} | {} | ${} | Qty: {}", 
                                                            data.market_id, data.outcome, data.price, data.size
                                                        );
                                                    }
                                                    Err(db_err) => {
                                                        error!("DB Error: {}", db_err);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    // Pongs, heartbeats, or other non-documented messages
                                }
                            }
                        }
                        Ok(Message::Ping(ping_data)) => {
                            let _ = write.send(Message::Pong(ping_data)).await;
                        }
                        Ok(Message::Close(c)) => {
                            warn!("WS closed: {:?}", c);
                            break;
                        }
                        Err(e) => {
                            error!("WS stream error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                error!("WS connection failed: {}. Retrying...", e);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}
