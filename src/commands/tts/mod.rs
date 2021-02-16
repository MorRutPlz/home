pub mod say;

use serenity::{
    client::Context,
    framework::standard::{
        macros::{check, group},
        Args, CommandOptions, Reason,
    },
    model::channel::Message,
};

use crate::commands::tts::say::*;
use crate::commands::*;

#[group]
#[prefix = "tts"]
#[description = "Group of commands for TTS stuff."]
#[summary = "Commands for TTS stuff"]
#[commands(say)]
#[checks(Channel, TTS)]
struct TTS;

#[check]
#[name = "TTS"]
async fn tts_check(
    _ctx: &Context,
    _msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    Ok(())
}
