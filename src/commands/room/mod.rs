pub mod add;
pub mod create;
pub mod remove;

use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::{ApplicationCommandOptionType, Interaction},
    },
};

pub async fn register(ctx: &Context, guild_id: GuildId, application_id: u64) {
    Interaction::create_guild_application_command(&ctx.http, guild_id, application_id, |a| {
        a.name("room")
            .description("Subcommands for room management")
            .create_interaction_option(|o| {
                o.name("add")
                    .description("Adds a user to your room")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|o| {
                        o.name("user")
                            .description("The user to be added")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
            })
            .create_interaction_option(|o| {
                o.name("remove")
                    .description("Removes a user from your room")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|o| {
                        o.name("user")
                            .description("The user to be removed")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
            })
            .create_interaction_option(|o| {
                o.name("create")
                    .description("Creates a room for a user")
                    .kind(ApplicationCommandOptionType::SubCommand)
                    .create_sub_option(|o| {
                        o.name("user")
                            .description("The new owner of this room")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|o| {
                        o.name("room_name")
                            .description("The name of the room and the permission role")
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
            "add" => add::execute(ctx, interaction).await,
            "create" => create::execute(ctx, interaction).await,
            "remove" => remove::execute(ctx, interaction).await,
            _ => {}
        },
        None => {}
    }
}
