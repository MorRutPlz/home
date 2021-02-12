use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Colour,
};

use crate::room::RoomsTMK;

#[command]
#[description = "Removes a user from your room"]
#[usage = "<@mention user here>"]
#[only_in("guild")]
async fn remove(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() > 0 {
        let data = ctx.data.read().await;
        let rooms = data.get::<RoomsTMK>().unwrap();
        let role_id = rooms.get(&msg.author.id).unwrap().1;

        let mut error = None;
        let mut errors = Vec::new();
        let mut not_in_room = Vec::new();
        let mut success = Vec::new();

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
                        Err(e) => println!("failed to send message: {}", e),
                    }

                    return Ok(());
                }

                continue;
            }

            match user
                .has_role(&ctx.http, msg.guild_id.unwrap(), role_id)
                .await
            {
                Ok(true) => {
                    match ctx
                        .http
                        .remove_member_role(msg.guild_id.unwrap().0, user.id.0, role_id.0)
                        .await
                    {
                        Ok(_) => success.push(user.tag()),
                        Err(e) => {
                            println!("failed to remove user from role: {}", e);
                            errors.push(user.tag());
                            error = Some(e.to_string());
                        }
                    }
                }
                Ok(false) => not_in_room.push(user.tag()),
                Err(e) => {
                    println!("failed to check if user has role: {}", e);
                    errors.push(user.tag());
                    error = Some(e.to_string());
                }
            }
        }

        match msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg).embed(|e| {
                    e.title("Command Execution ⚙️");

                    if success.len() > 0 {
                        e.field(
                            "Successfully removed:",
                            {
                                let mut result = String::new();

                                for i in 0..success.len() {
                                    result.push_str(&success[i]);

                                    if i != success.len() - 1 {
                                        result.push_str("\n");
                                    }
                                }

                                result
                            },
                            false,
                        );
                    }

                    if not_in_room.len() > 0 {
                        e.field(
                            "Wasn't in the room in the first place:",
                            {
                                let mut result = String::new();

                                for i in 0..not_in_room.len() {
                                    result.push_str(&not_in_room[i]);

                                    if i != not_in_room.len() - 1 {
                                        result.push_str("\n");
                                    }
                                }

                                result
                            },
                            false,
                        );
                    }

                    if errors.len() > 0 {
                        e.field(
                            "Failed to add:",
                            {
                                let mut result = String::new();

                                for i in 0..errors.len() {
                                    result.push_str(&errors[i]);

                                    result.push_str("\n");
                                }

                                result.push_str("\n");
                                result.push_str("**Error**: ");
                                result.push_str(error.as_ref().unwrap());
                                result
                            },
                            false,
                        );
                    }

                    e.color(Colour(16748258))
                })
            })
            .await
        {
            Ok(_) => {}
            Err(e) => println!("failed to send message: {}", e),
        }
    } else {
        match msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg).embed(|e| {
                    e.color(Colour(10038562));
                    e.description(
                        "**Error**: You need to tag a user! Do ``/help room remove`` for more info.",
                    )
                })
            })
            .await
        {
            Ok(_) => {}
            Err(e) => println!("failed to send message: {}", e),
        }
    }

    Ok(())
}
