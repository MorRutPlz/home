use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Colour,
};

use crate::{error, typemap::TypeMapSharedCache};

#[command]
#[description = "Adds a user to your room"]
#[usage = "<@mention user here>"]
async fn add(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() > 0 {
        let data = ctx.data.read().await;
        let cache = data.get::<TypeMapSharedCache>().unwrap();
        let role_id = cache
            .get_user_room_map()
            .get(&msg.author.id)
            .unwrap()
            .role_id;

        let mut already_added = String::new();
        let mut errors = String::new();
        let mut success = String::new();
        let mut not_added = String::new();

        for user in msg.mentions.iter() {
            if user.id.0 == 807187286696787969 {
                if msg.mentions.len() == 1 {
                    match msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.reference_message(msg).embed(|e| {
                                e.color(Colour(10038562));
                                e.description("**Error**: Stop ;-;")
                            })
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => error!("failed to send message: {}", e),
                    }

                    return Ok(());
                }

                not_added.push_str(&user.tag());
                not_added.push_str("\n");
                continue;
            }

            if user.id == msg.author.id {
                if msg.mentions.len() == 1 {
                    match msg
                        .channel_id
                        .send_message(&ctx.http, |m| {
                            m.reference_message(msg).embed(|e| {
                                e.color(Colour(10038562));
                                e.description("**Error**: You can't add yourself to your room!")
                            })
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => error!("failed to send message: {}", e),
                    }

                    return Ok(());
                }

                not_added.push_str(&user.tag());
                not_added.push_str("\n");
                continue;
            }

            match user
                .has_role(&ctx.http, msg.guild_id.unwrap(), role_id)
                .await
            {
                Ok(true) => already_added.push_str(&format!("{}\n", user.tag())),
                Ok(false) => {
                    match ctx
                        .http
                        .add_member_role(msg.guild_id.unwrap().0, user.id.0, role_id)
                        .await
                    {
                        Ok(_) => success.push_str(&format!("{}\n", user.tag())),
                        Err(e) => {
                            error!("failed to add user to role: {}", e);
                            errors.push_str(&user.tag());
                            errors.push_str("\n");
                        }
                    }
                }
                Err(e) => {
                    error!("failed to check if user has role: {}", e);
                    errors.push_str(&user.tag());
                    errors.push_str("\n");
                }
            }
        }

        match msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg).embed(|e| {
                    e.title("Command Execution ⚙️");

                    if success.len() > 0 {
                        e.field("Successfully added:", success, false);
                    }

                    if already_added.len() > 0 {
                        e.field("Already in room:", already_added, false);
                    }

                    if not_added.len() > 0 {
                        e.field("Not added:", not_added, false);
                    }

                    if errors.len() > 0 {
                        e.field("Failed to add:", errors, false);
                    }

                    e.color(Colour(16748258))
                })
            })
            .await
        {
            Ok(_) => {}
            Err(e) => error!("failed to send message: {}", e),
        }
    } else {
        match msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg).embed(|e| {
                    e.color(Colour(10038562));
                    e.description(
                        "**Error**: You need to tag a user! Do ``/help room add`` for more info.",
                    )
                })
            })
            .await
        {
            Ok(_) => {}
            Err(e) => error!("failed to send message: {}", e),
        }
    }

    Ok(())
}
