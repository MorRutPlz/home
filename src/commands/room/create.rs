use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Colour,
};

use crate::commands::room::*;

#[command]
#[description = "Creates a room for a user if it doesn't exist"]
#[only_in("guild")]
#[checks(Sudo)]
async fn create(ctx: &Context, msg: &Message) -> CommandResult {
    match msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.reference_message(msg).embed(|e| {
                e.color(Colour(10038562));
                e.description("**Error**: Command still a WIP")
            })
        })
        .await
    {
        Ok(_) => {}
        Err(e) => error!("failed to send message: {}", e),
    }

    Ok(())
}
