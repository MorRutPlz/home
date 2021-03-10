use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Colour,
};

use crate::commands::room::*;

#[command]
#[description = "Lists users and rooms"]
#[only_in("guild")]
#[checks(Sudo)]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    // This command should DM the user instead of showing the list in public

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
