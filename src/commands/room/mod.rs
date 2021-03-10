pub mod add;
pub mod create;
pub mod list;
pub mod remove;

use serenity::{
    client::Context,
    framework::standard::{
        macros::{check, group},
        Args, CommandOptions, Reason,
    },
    model::channel::Message,
    utils::Colour,
};

use crate::commands::*;
use crate::{
    commands::room::{add::*, create::*, list::*, remove::*},
    typemap::TypeMapSharedCache,
};

#[group]
#[prefix = "room"]
#[description = "Group of commands for room management."]
#[summary = "Commands for room stuff"]
#[commands(add, remove, list, create)]
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
    let cache = data.get::<TypeMapSharedCache>().unwrap();

    match cache.get_user_room_map().contains_key(&msg.author.id) {
        true => Ok(()),
        false => {
            match msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.reference_message(msg).embed(|e| {
                        e.color(Colour(10038562));
                        e.description(
                            "**Error**: You don't seem to be registered. Try leaving and rejoining the server or asking a mod to fix this",
                        )
                    })
                })
                .await
            {
                Ok(_) => {}
                Err(e) => error!("failed to send message: {}", e),
            }

            Err(Reason::User("User not registered".to_string()))
        }
    }
}
