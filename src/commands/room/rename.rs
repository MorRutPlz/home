use serde_json::{json, Map};
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::interactions::{Interaction, InteractionResponseType},
    utils::Colour,
};
use std::iter::FromIterator;

use crate::{commands::get_option, error, typemap::TypeMapSharedCache};

pub async fn execute(ctx: Context, interaction: Interaction) {
    let data = ctx.data.read().await;
    let cache = data.get::<TypeMapSharedCache>().unwrap();

    let rooms = match cache.get_user_room_map().get(&interaction.member.user.id) {
        Some(n) => n.clone(),
        None => {
            match interaction
                .create_interaction_response(&ctx.http, |m| {
                    m.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|d| d.embed(|e| {
                        e.colour(Colour(10038562));
                        e.description(
                            "**Error**: You don't have any rooms! Try leaving the server and rejoining or consulting this with a mod",
                        )
                    }))
                })
                .await
            {
                Ok(_) => {}
                Err(e) => error!("failed to create interaction response: {}", e),
            }

            return;
        }
    };

    drop(cache);

    let room_name = match get_option(0, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => {
                    let n = n
                        .chars()
                        .into_iter()
                        .filter(|x| x.is_alphanumeric())
                        .collect::<String>();

                    if n.len() <= 83 && n.len() >= 1 {
                        n
                    } else {
                        let msg = format!(
                            "**Error**: Room name has to be between 1 and 83 characters long!"
                        );

                        match interaction
                            .create_interaction_response(&ctx.http, |m| {
                                m.kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|d| {
                                        d.embed(|e| e.color(Colour(10038562)).description(msg))
                                    })
                            })
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => error!("failed to create interaction response: {}", e),
                        }

                        return;
                    }
                }
                None => return,
            },
            None => return,
        },
        None => return,
    };

    let mut embed = CreateEmbed::default();

    if rooms.contains(&interaction.channel_id) {
        match ctx
            .http
            .edit_channel(
                interaction.channel_id.0,
                &Map::from_iter(
                    [("name".to_string(), json!(room_name))]
                        .iter()
                        .map(|x| x.to_owned()),
                ),
            )
            .await
        {
            Ok(_) => {
                embed
                    .color(Colour(16748258))
                    .description("Changed room name :3");
            }
            Err(e) => {
                error!("failed to edit channel name: {}", e);

                embed
                    .color(Colour(10038562))
                    .description(format!("**Error**: Failed to edit channel name: {}", e));
            }
        }
    } else {
        embed
            .color(Colour(10038562))
            .description("**Error**: You aren't in a room that you own!");
    }

    match interaction
        .create_interaction_response(&ctx.http, |m| {
            m.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.set_embed(embed))
        })
        .await
    {
        Ok(_) => {}
        Err(e) => error!("failed to create interaction response: {}", e),
    }
}
