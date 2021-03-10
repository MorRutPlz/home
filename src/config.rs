use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub discord: Discord,
    pub guild_id: u64,
    pub main_server: u64,
    pub moderation_server: u64,
    pub mongodb: MongoDB,
    pub public_channels: Vec<u64>,
    pub status: String,
    pub sudoers: Vec<u64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Discord {
    pub token: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MongoDB {
    pub connection_string: String,
}
