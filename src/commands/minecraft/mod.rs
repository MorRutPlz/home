mod link;

use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::{ApplicationCommandOptionType, Interaction},
    },
};

pub async fn register(ctx: &Context, guild_id: GuildId, application_id: u64) {
    Interaction::create_guild_application_command(&ctx.http, guild_id, application_id, |a| {
        a.name("minecraft")
            .description("Subcommands for Minecraft linking")
            .create_interaction_option(|o| {
                o.name("link")
                    .description("Generates a new server address you join to link your account")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|o| {
                        o.name("username")
                            .description("Current Minecraft username")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
    })
    .await
    .unwrap();
}

pub async fn execute(ctx: Context, interaction: Interaction) {
    match interaction.data.as_ref().unwrap().options.get(0) {
        Some(n) => match n.name.as_str() {
            "link" => link::execute(ctx, interaction).await,
            _ => {}
        },
        None => {}
    }
}
