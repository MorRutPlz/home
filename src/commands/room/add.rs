use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{
        id::{RoleId, UserId},
        interactions::{Interaction, InteractionResponseType},
    },
    utils::Colour,
};

use crate::{error, typemap::TypeMapSharedCache};

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

    let to_be_added = UserId(
        interaction
            .data
            .as_ref()
            .unwrap()
            .options
            .get(0)
            .unwrap()
            .options
            .get(0)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
    );

    let mut embed = CreateEmbed::default();

    // Don't let them add the Home bot or themselves in a room
    if to_be_added.0 != ctx.http.get_current_user().await.unwrap().id.0
        && to_be_added != interaction.member.user.id
    {
        // TODO: Support for multiple rooms here
        let room = &rooms[0];

        // Get user
        match ctx.http.get_user(to_be_added.0).await {
            Ok(user) => {
                match user
                    .has_role(&ctx.http, interaction.guild_id, RoleId(room.role_id))
                    .await
                {
                    Ok(true) => {
                        embed
                            .color(Colour(16748258))
                            .description("Already added that user to your room!");
                    }
                    Ok(false) => {
                        match ctx
                            .http
                            .add_member_role(interaction.guild_id.0, to_be_added.0, room.role_id)
                            .await
                        {
                            Ok(_) => {
                                embed
                                    .color(Colour(16748258))
                                    .description("Added user to room :3");
                            }
                            Err(e) => {
                                error!("failed to add user to role: {}", e);

                                embed.color(Colour(10038562)).description(format!(
                                    "**Error**: Failed to add user to role: {}",
                                    e
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        error!("failed to check if user has role: {}", e);

                        embed.color(Colour(10038562)).description(format!(
                            "**Error**: Failed to check if user already has role: {}",
                            e
                        ));
                    }
                }
            }
            Err(e) => {
                error!("failed to get user from user id: {}", e);

                embed
                    .color(Colour(10038562))
                    .description(format!("**Error**: Failed to get user object: {}", e));
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
