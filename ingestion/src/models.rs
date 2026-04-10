use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub market_id: String,
    pub outcome: String,
    pub price: f64,
    pub size: f64,
    pub timestamp: DateTime<Utc>,
}

/// The structure of a message from the Polymarket WS Market channel.
/// It can be a list of events.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ws_message {
    Events(Vec<PolymarketEvent>),
    SingleEvent(PolymarketEvent),
}

#[derive(Debug, Deserialize)]
pub struct PolymarketEvent {
    pub event_type: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub outcome: String,
    
    // Using custom deserialization because price/size can be strings
    #[serde(default, deserialize_with = "deserialize_f64_opt")]
    pub price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_f64_opt")]
    pub size: Option<f64>,
    
    #[serde(default)]
    pub timestamp: String, // String ISO format
}

fn deserialize_f64_opt<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        String(String),
        Float(f64),
    }

    match Option::<StringOrFloat>::deserialize(deserializer)? {
        Some(StringOrFloat::Float(f)) => Ok(Some(f)),
        Some(StringOrFloat::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
}
