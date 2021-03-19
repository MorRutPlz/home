use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{
        channel::{Channel, PermissionOverwrite, PermissionOverwriteType},
        id::{ChannelId, UserId},
        interactions::{Interaction, InteractionResponseType},
        Permissions,
    },
    utils::Colour,
};

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

    let to_be_added = match get_option(0, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => match n.parse() {
                    Ok(n) => UserId(n),
                    Err(_) => return,
                },
                None => return,
            },
            None => return,
        },
        None => return,
    };

    let room = match get_option(1, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => match n.parse() {
                    Ok(n) => {
                        if rooms.contains(&ChannelId(n)) {
                            Some(ChannelId(n))
                        } else {
                            match interaction
                                .create_interaction_response(&ctx.http, |m| {
                                    m.kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|d| {
                                            d.embed(|e| {
                                                e.color(Colour(10038562)).description(format!(
                                                    "**Error**: That channel is not one of your rooms!"
                                                ))
                                            })
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
                    Err(_) => return,
                },
                None => return,
            },
            None => return,
        },
        None => {
            if rooms.len() > 1 {
                let msg = format!(
                    "**Error**: Since you have multiple rooms, you have to specify a room!"
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

            None
        }
    };

    let mut embed = CreateEmbed::default();

    // Don't let them add the Home bot or themselves in a room
    if to_be_added.0 != ctx.http.get_current_user().await.unwrap().id.0
        && to_be_added != interaction.member.user.id
    {
        let room = match room {
            Some(n) => n,
            None => rooms[0],
        };

        match ctx.http.get_channel(room.0).await {
            Ok(Channel::Guild(n)) => {
                if n.permission_overwrites
                    .iter()
                    .filter(|x| match x.kind {
                        PermissionOverwriteType::Member(n) => {
                            if n == to_be_added {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    })
                    .collect::<Vec<_>>()
                    .len()
                    == 0
                {
                    match n
                        .create_permission(
                            &ctx.http,
                            &PermissionOverwrite {
                                allow: Permissions::READ_MESSAGES,
                                deny: Permissions::empty(),
                                kind: PermissionOverwriteType::Member(to_be_added),
                            },
                        )
                        .await
                    {
                        Ok(_) => {
                            embed
                                .color(Colour(16748258))
                                .description("Added user to room :3");
                        }
                        Err(e) => {
                            error!("failed to add user to channel: {}", e);

                            embed.color(Colour(10038562)).description(format!(
                                "**Error**: Failed to add user to channel: {}",
                                e
                            ));
                        }
                    }
                } else {
                    embed
                        .color(Colour(10038562))
                        .description(format!("**Error**: That user is already in your room!"));
                }
            }
            Ok(_) => {
                error!("expected a guild channel");

                embed
                    .color(Colour(10038562))
                    .description(format!("**Error**: Expected a guild channel"));
            }
            Err(e) => {
                error!("failed to get channel: {}", e);

                embed
                    .color(Colour(10038562))
                    .description(format!("**Error**: Failed to get channel: {}", e));
            }
        }
    } else {
        embed
            .color(Colour(10038562))
            .description("**Error**: Stop ;-;");
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
