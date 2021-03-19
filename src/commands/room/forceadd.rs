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

use crate::{
    commands::{get_option, sudo_check},
    error,
};

pub async fn execute(ctx: Context, interaction: Interaction) {
    if !sudo_check(&ctx, &interaction).await {
        return;
    }

    let channel = match get_option(0, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => match n.parse() {
                    Ok(n) => ChannelId(n),
                    Err(_) => return,
                },
                None => return,
            },
            None => return,
        },
        None => return,
    };

    let to_be_added = match get_option(1, &interaction) {
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

    let mut embed = CreateEmbed::default();

    match ctx.http.get_channel(channel.0).await {
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
                            .description("Added user to channel :3");
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
                    .description(format!("**Error**: That user is already in the channel!"));
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
