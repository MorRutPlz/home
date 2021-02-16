mod commands;
mod config;
mod logger;
mod rooms;
mod tts;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::StandardFramework,
    model::prelude::{Activity, OnlineStatus, Ready},
};

use crate::commands::room::*;
use crate::commands::tts::*;
use crate::commands::*;
use crate::config::{Config, Discord, Google};
use crate::logger::info;
use crate::tts::LastTTS;
use songbird::SerenityInit;
use std::fs;
use std::io::ErrorKind;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = ctx.data.read().await;
        let config = data.get::<ConfigTMK>().unwrap();

        let activity = Activity::playing(&config.status);
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;

        info("Bot started! <3");
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match fs::read_to_string("config.toml") {
        Ok(n) => match toml::from_str::<Config>(&n) {
            Ok(n) => n,
            Err(e) => panic!("failed to parse config.toml: {}", e),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let config = Config {
                    guild_id: 806947535825403904,
                    public_channels: vec![],
                    status: "bunni is pog~".to_string(),
                    discord: Discord {
                        token: "".to_string(),
                    },
                    google: Google {
                        access_token: "".to_string(),
                        api_key: "".to_string(),
                    },
                };

                fs::write("config.toml", toml::to_string_pretty(&config).unwrap()).unwrap();

                config
            }
            _ => panic!("failed to read config.toml: {}", e),
        },
    };

    let rooms = match fs::read_to_string("rooms.toml") {
        Ok(n) => match toml::from_str::<rooms::RoomsVec>(&n) {
            Ok(n) => n,
            Err(e) => panic!("failed to parse rooms.toml: {}", e),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let rooms = rooms::RoomsVec {
                    room: vec![rooms::RoomInfo {
                        user_id: vec![0],
                        channel_id: 0,
                        role_id: 0,
                    }],
                };

                fs::write("rooms.toml", toml::to_string_pretty(&rooms).unwrap()).unwrap();

                rooms
            }
            _ => panic!("failed to read rooms.toml: {}", e),
        },
    };

    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("/"))
        .help(&MY_HELP)
        .group(&ROOM_GROUP)
        .group(&TTS_GROUP);

    let mut client = Client::builder(&config.discord.token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<ConfigTMK>(config);
        data.insert::<LastTTS>(None);
        data.insert::<RoomsTMK>(rooms.into());
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
