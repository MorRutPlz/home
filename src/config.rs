use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub guild_id: u64,
    pub public_channels: Vec<u64>,
    pub status: String,
    pub discord: Discord,
    pub google: Google,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Discord {
    pub token: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Google {
    pub access_token: String,
    pub api_key: String,
}
