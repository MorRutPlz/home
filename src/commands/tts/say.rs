use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Says a message out loud in your current VC"]
#[usage = "<message>"]
#[only_in("guild")]
async fn say(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}
