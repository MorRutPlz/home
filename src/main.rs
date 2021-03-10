mod commands;
mod config;
mod handler;
mod logger;
mod model;
mod shared_cache;
mod typemap;

use mongodb::Client as MongoClient;
use serenity::{client::Client, framework::standard::StandardFramework};
use shared_cache::SharedCache;
use std::fs;
use std::io::ErrorKind;
use typemap::{TypeMapConfig, TypeMapSharedCache};

use crate::commands::*;
use crate::commands::{me::*, room::*};
use crate::config::{Config, Discord, MongoDB};
use crate::handler::Handler;

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
                    discord: Discord {
                        token: "".to_string(),
                    },
                    guild_id: 806947535825403904,
                    main_server: 806947535825403904,
                    moderation_server: 818026699349688331,
                    mongodb: MongoDB {
                        connection_string: "".to_string(),
                    },
                    public_channels: vec![],
                    status: "bunni is pog~".to_string(),
                    sudoers: vec![],
                };

                fs::write("config.toml", toml::to_string_pretty(&config).unwrap()).unwrap();

                config
            }
            _ => panic!("failed to read config.toml: {}", e),
        },
    };

    let shared_cache = SharedCache::new(
        MongoClient::with_uri_str(&config.mongodb.connection_string)
            .await
            .unwrap()
            .database("home"),
    )
    .await;

    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("/"))
        .help(&MY_HELP)
        .group(&ROOM_GROUP)
        .group(&ME_GROUP);

    let mut client = Client::builder(&config.discord.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<TypeMapConfig>(config);
        data.insert::<TypeMapSharedCache>(shared_cache);
    }

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
