mod codec;
mod model;

use feather_protocol::{
    packets::{
        client::{Handshake, LoginStart},
        server::DisconnectLogin,
    },
    ClientHandshakePacket, ClientLoginPacket, ServerLoginPacket,
};
use num_bigint::BigInt;
use serde_json::json;
use serenity::{http::Http, model::id::UserId};
use sha1::{Digest, Sha1};
use std::thread;
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::task;

use self::codec::MinecraftCodec;
use self::model::AuthResponse;
use crate::{debug, error, info, warn};

pub fn start_listener(http: Arc<Http>) -> Arc<Mutex<HashMap<String, (UserId, String)>>> {
    let verification_codes = Arc::new(Mutex::new(HashMap::new()));

    {
        let verification_codes = verification_codes.clone();

        thread::spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();
                info!("Listening in 0.0.0.0:25565");

                loop {
                    debug!("Server loop cycle completed");

                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            debug!("New connection: {}", addr);

                            let verification_codes = verification_codes.clone();
                            let http = http.clone();

                            task::spawn(async move {
                                match handle_stream(
                                    MinecraftCodec::new(stream),
                                    verification_codes.clone(),
                                    http.clone(),
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        debug!("error while handling connection: {}", e)
                                    }
                                }
                            });
                        }
                        Err(_) => {}
                    }
                }
            })
        });
    }

    verification_codes
}

pub async fn handle_stream(
    mut stream: MinecraftCodec,
    codes: Arc<Mutex<HashMap<String, (UserId, String)>>>,
    http: Arc<Http>,
) -> Result<(), &'static str> {
    let handshake_packet: Handshake = match stream.next::<ClientHandshakePacket>().await? {
        ClientHandshakePacket::Handshake(n) => n,
    };
    let login_packet: LoginStart = match stream.next::<ClientLoginPacket>().await? {
        ClientLoginPacket::LoginStart(n) => n,
        _ => return Err("expected login start packet"),
    };
    let shared_secret = stream.enable_encryption().await?;

    let mut hasher = Sha1::new();
    hasher.update(b""); // server ID - always empty
    hasher.update(&shared_secret);
    hasher.update(&stream.rsa_public_key);

    let bigint = BigInt::from_signed_bytes_be(&hasher.finalize().as_slice());

    let response = match reqwest::get(&format!(
        "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
        login_packet.name,
        format!("{:x}", bigint)
    ))
    .await
    {
        Ok(n) => n
            .json::<AuthResponse>()
            .await
            .map_err(|_| "failed to deserialize auth response")?,
        Err(_) => {
            return Err("failed to send GET request to sessionserver.mojang.com");
        }
    };

    info!(
        "player {} authenticated with hostname {}:\n\n{:?}\n",
        response.name, handshake_packet.server_address, response
    );

    let mut codes = codes.lock().await;
    let code: String = handshake_packet
        .server_address
        .replace(".homebot.freemyip.com", "");

    let user = match codes.remove(&code) {
        Some((user, username)) => {
            if username == response.name {
                user
            } else {
                drop(codes);

                let message = format!(
                    "Username mismatch! This code was meant for the user {}",
                    username
                );

                stream
                    .send(&ServerLoginPacket::DisconnectLogin(DisconnectLogin {
                        reason: serde_json::to_string(&json!({ "text": message })).unwrap(),
                    }))
                    .await?;

                return Err("username mismatch");
            }
        }
        None => {
            drop(codes);

            let message = format!("Verification code {} doesn't exist or has been deleted (generating new codes will delete earlier ones)", code);

            stream
                .send(&ServerLoginPacket::DisconnectLogin(DisconnectLogin {
                    reason: serde_json::to_string(&json!({ "text": message })).unwrap(),
                }))
                .await?;

            return Err("verification code doesn't exist");
        }
    };

    drop(codes);

    stream
        .send(&ServerLoginPacket::DisconnectLogin(DisconnectLogin {
            reason: serde_json::to_string(&json!({ "text": "Successfully linked!" })).unwrap(),
        }))
        .await?;

    let msg = "ye ur verified congrats (this doesn't actually do anything as of rn so feel free to do this again)";

    match http.get_user(user.0).await {
        Ok(n) => match n.create_dm_channel(&http).await {
            Ok(n) => match n.send_message(&http, |m| m.content(msg)).await {
                Ok(_) => {}
                Err(e) => error!("failed to send message: {}", e),
            },
            Err(e) => error!("failed to create DM channel: {}", e),
        },
        Err(e) => error!("failed to get user: {}", e),
    }

    Ok(())
}
