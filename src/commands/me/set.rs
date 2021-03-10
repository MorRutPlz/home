use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Colour,
};

use crate::error;

#[command]
#[description = "Sets a property for your profile"]
#[only_in("guild")]
async fn set(ctx: &Context, msg: &Message) -> CommandResult {
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
