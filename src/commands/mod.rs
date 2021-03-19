mod minecraft;
mod room;

use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::{
            ApplicationCommandInteractionDataOption, Interaction, InteractionResponseType,
        },
    },
    utils::Colour,
};

use crate::{error, typemap::TypeMapConfig};

pub async fn register_commands(ctx: &Context) {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    let guild_id = GuildId(config.guild_id);
    let application_id = ctx.http.get_current_user().await.unwrap().id.0;

    drop(config);

    Interaction::create_guild_application_command(&ctx.http, guild_id, application_id, |a| {
        a.name("help")
            .description("Lists commands for using this bot")
    })
    .await
    .unwrap();

    minecraft::register(ctx, guild_id, application_id).await;
    room::register(ctx, guild_id, application_id).await;
}

pub async fn execute(ctx: Context, interaction: Interaction) {
    match interaction.data.as_ref().unwrap().name.as_str() {
        "help" => {
            match interaction
                .create_interaction_response(&ctx.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            d.embed(|e| {
                                e.title("List of commands & subcommands")
                                    .colour(Colour(16178136))
                                    .field(
                                        "Room",
                                        "Usage: ``/room <subcommand>``\n\n\
                                    ``add``\n\
                                    ``create``\n\
                                    ``remove``",
                                        true,
                                    )
                                    .field(
                                        "Minecraft",
                                        "Usage: ``/minecraft <subcommand>``\n\n\
                                    ``link``",
                                        true,
                                    )
                            })
                        })
                })
                .await
            {
                Ok(_) => {}
                Err(e) => error!("failed to create interaction response for 'help': {}", e),
            }
        }
        "minecraft" => minecraft::execute(ctx, interaction).await,
        "room" => room::execute(ctx, interaction).await,
        _ => {}
    }
}

pub fn get_option(
    index: usize,
    interaction: &Interaction,
) -> Option<&ApplicationCommandInteractionDataOption> {
    interaction
        .data
        .as_ref()
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(index)
}

pub async fn sudo_check(ctx: &Context, interaction: &Interaction) -> bool {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    config.sudoers.contains(&interaction.member.user.id.0)
}
