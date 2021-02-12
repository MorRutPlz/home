use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub public_channels: Vec<u64>,
    pub status: String,
    pub token: String,
}
