pub mod add;
pub mod remove;

use serenity::{
    client::Context,
    framework::standard::{
        macros::{check, group},
        Args, CommandOptions, Reason,
    },
    model::{
        channel::Message,
        id::{ChannelId, RoleId, UserId},
    },
    prelude::TypeMapKey,
    utils::Colour,
};

use crate::commands::room::{add::*, remove::*};
use crate::commands::*;
use std::collections::HashMap;

pub struct RoomsTMK;

impl TypeMapKey for RoomsTMK {
    type Value = HashMap<UserId, (ChannelId, RoleId)>;
}

#[group]
#[prefix = "room"]
#[description = "Group of commands for room management."]
#[summary = "Commands for room stuff"]
#[commands(add, remove)]
#[checks(Channel, Room)]
struct Room;

#[check]
#[name = "Room"]
async fn room_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let rooms = data.get::<RoomsTMK>().unwrap();

    match rooms.contains_key(&msg.author.id) {
        true => Ok(()),
        false => {
            match msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.reference_message(msg).embed(|e| {
                        e.color(Colour(10038562));
                        e.description(
                            "**Error**: You don't seem to be registered. Are you new here?",
                        )
                    })
                })
                .await
            {
                Ok(_) => {}
                Err(e) => println!("failed to send message: {}", e),
            }

            Err(Reason::User("User not registered".to_string()))
        }
    }
}
