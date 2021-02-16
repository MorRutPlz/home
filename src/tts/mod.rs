mod model;

use crate::commands::ConfigTMK;
use crate::logger::{error, info};
use crate::tts::model::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use reqwest::{header::AUTHORIZATION, Client};
use serenity::{client::Context, model::channel::Message};
use serenity::{model::channel::ChannelType, prelude::TypeMapKey};
use songbird::input::{Codec, Container, Input};
use std::{
    io::Cursor,
    time::{Duration, SystemTime},
};
use tokio::time::sleep;

pub struct LastTTS;

impl TypeMapKey for LastTTS {
    type Value = Option<SystemTime>;
}

pub async fn _handle_tts(ctx: Context, message: Message) {
    match message.guild_id.unwrap().channels(&ctx.http).await {
        Ok(n) => {
            let mut channel_id = None;

            while let Some(channel) = n.values().next() {
                match channel.kind {
                    ChannelType::Voice => match channel.members(&ctx.cache).await {
                        Ok(n) => {
                            if n.into_iter()
                                .filter(|x| x.user.id == message.author.id)
                                .collect::<Vec<_>>()
                                .len()
                                > 0
                            {
                                channel_id = Some(channel.id);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("failed to get : {}", e)
                        }
                    },
                    _ => {}
                }
            }

            // Not in VC
            match channel_id {
                Some(_n) => {}
                None => {}
            }
        }
        Err(_e) => {}
    }

    let mut data = ctx.data.write().await;
    let config = data.get::<ConfigTMK>().unwrap().to_owned();
    let last = data.get_mut::<LastTTS>().unwrap();
    let client = Client::new();
    let _user = message.author.id;

    *last = Some(SystemTime::now());

    let tts = TTSRequest {
        audio_config: AudioConfig {
            audio_encoding: AudioEncoding::Linear16,
            speaking_rate: None,
            pitch: None,
            volume_gain_db: None,
            sample_rate_hertz: None,
            effects_profile_id: None,
        },
        input: SynthesisInput::Text(message.content),
        voice: VoiceSelectionParams {
            language_code: "en-US".to_string(),
            name: Some("en-US-Wavenet-D".to_string()),
            ssml_gender: Some(SsmlVoiceGender::Male),
        },
    };

    info("handling tts message");

    let response = match client
        .post(&format!(
            "https://texttospeech.googleapis.com/v1/text:synthesize?key={}",
            config.google.api_key
        ))
        .header(
            AUTHORIZATION,
            &format!("Bearer {}", config.google.access_token),
        )
        .json(&tts)
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(n) => match serde_json::from_str::<TTSResponse>(&n) {
                Ok(n) => n,
                Err(e) => {
                    error(format!("error 3: {}", e));
                    info(format!("got response: {}", n));

                    return;
                }
            },
            Err(e) => {
                error(format!("error 2: {}", e));
                return;
            }
        },
        Err(e) => {
            error(format!("error 1: {}", e));
            return;
        }
    };

    info("got tts response object");
    drop(data);

    match base64::decode(&response.audio_content) {
        Ok(raw) => {
            let mut raw = Cursor::new(raw);
            let mut output = Cursor::new(Vec::new());

            if let Ok(mut last) = raw.read_i16::<LittleEndian>() {
                output.write_i16::<LittleEndian>(last).unwrap();

                loop {
                    match raw.read_i16::<LittleEndian>() {
                        Ok(x) => {
                            output
                                .write_i16::<LittleEndian>(((x as i32 + last as i32) / 2) as i16)
                                .unwrap();
                            output.write_i16::<LittleEndian>(x).unwrap();

                            last = x;
                        }
                        Err(_) => break,
                    }
                }
            }

            match songbird::get(&ctx).await {
                Some(voice) => match voice.get(config.guild_id) {
                    Some(call) => {
                        call.lock().await.play_source(Input::new(
                            false,
                            output.into_inner().into(),
                            Codec::Pcm,
                            Container::Raw,
                            None,
                        ));
                    }
                    None => {
                        let (call, _) = voice.join(config.guild_id, 807103921549082624).await;

                        call.lock().await.play_source(Input::new(
                            false,
                            output.into_inner().into(),
                            Codec::Pcm,
                            Container::Raw,
                            None,
                        ));
                    }
                },
                None => error("error 5: failed to get songbird voice client"),
            }
        }
        Err(e) => error(format!("error 4: {}", e)),
    }
}

pub async fn _disconnect_loop(ctx: Context) {
    let data = ctx.data.read().await;
    let config = data.get::<ConfigTMK>().unwrap().to_owned();

    drop(data);

    loop {
        sleep(Duration::from_secs(1)).await;

        let mut data = ctx.data.write().await;
        let last = data.get_mut::<LastTTS>().unwrap();

        match last.as_ref() {
            Some(n) => {
                if n.elapsed().unwrap().as_secs() > 120 {
                    match songbird::get(&ctx).await {
                        Some(voice) => match voice.get(config.guild_id) {
                            Some(call) => {
                                let _ = call.lock().await.leave().await;
                            }
                            None => {}
                        },
                        None => {}
                    }

                    *last = None;
                }
            }
            None => {}
        }
    }
}
