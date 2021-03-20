use serde_json::{json, Map};
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{
        channel::{PermissionOverwrite, PermissionOverwriteType},
        id::{RoleId, UserId},
        interactions::{Interaction, InteractionResponseType},
        Permissions,
    },
    utils::Colour,
};
use std::iter::FromIterator;

use crate::{
    commands::{get_option, sudo_check},
    error,
    typemap::TypeMapSharedCache,
};

pub async fn execute(ctx: Context, interaction: Interaction) {
    if !sudo_check(&ctx, &interaction).await {
        return;
    }

    let owner = match get_option(0, &interaction) {
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

    let room_name = match get_option(1, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => n,
                None => return,
            },
            None => return,
        },
        None => return,
    };

    let mut embed = CreateEmbed::default();

    match create_room(&ctx, owner, format!("°•°♡{}’s-room♡°•°", room_name)).await {
        Ok(_) => {
            embed.color(Colour(16748258)).description("Created room :3");
        }
        Err(e) => embed = e,
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

pub async fn create_room(
    ctx: &Context,
    owner: UserId,
    room_name: String,
) -> Result<(), CreateEmbed> {
    let channel_overrides = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::READ_MESSAGES,
            kind: PermissionOverwriteType::Role(RoleId(806947535825403904)),
        },
        PermissionOverwrite {
            allow: Permissions::READ_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(owner),
        },
    ];

    let channel_name = format!("°•°♡{}’s-room♡°•°", room_name);

    match ctx.http.get_channel(821991091703119883).await {
        Ok(n) => match n.position() {
            Some(n) => {
                match ctx
                    .http
                    .create_channel(
                        806947535825403904,
                        &Map::from_iter(
                            [
                                ("name".to_string(), json!(channel_name)),
                                ("type".to_string(), json!(0)),
                                ("position".to_string(), json!(n - 1)),
                                ("parent_id".to_string(), json!("813785080421548103")),
                                (
                                    "permission_overwrites".to_string(),
                                    json!(channel_overrides),
                                ),
                            ]
                            .iter()
                            .map(|x| x.to_owned()),
                        ),
                    )
                    .await
                {
                    Ok(n) => {
                        let mut data = ctx.data.write().await;
                        let cache = data.get_mut::<TypeMapSharedCache>().unwrap();

                        match cache.add_user_room(owner, n.id).await {
                            Ok(_) => Ok(()),
                            Err(_) => Err(CreateEmbed::default()
                                .color(Colour(10038562))
                                .description(format!(
                                    "**Error**: Database error. Report this to the mods"
                                ))
                                .to_owned()),
                        }
                    }
                    Err(e) => {
                        error!("failed to create channel for room: {}", e);

                        Err(CreateEmbed::default()
                            .color(Colour(10038562))
                            .description(format!(
                                "**Error**: Failed to create channel for room: {}",
                                e
                            ))
                            .to_owned())
                    }
                }
            }
            None => {
                error!("marker channel of unknown type! could not get it's position");

                Err(CreateEmbed::default()
                    .color(Colour(10038562))
                    .description(format!(
                        "**Error**: Marker channel of unknown type! Could not get it's position"
                    ))
                    .to_owned())
            }
        },
        Err(e) => {
            error!("could not get marker channel: {}", e);

            Err(CreateEmbed::default()
                .color(Colour(10038562))
                .description(format!("**Error**: Failed to get marker channel: {}", e))
                .to_owned())
        }
    }
}
