pub mod me;
pub mod room;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{check, help},
        Args, CommandGroup, CommandOptions, CommandResult, HelpBehaviour, HelpOptions, Reason,
    },
    model::{channel::Message, id::UserId},
    utils::Colour,
};

use std::collections::HashSet;

use crate::{error, typemap::TypeMapConfig};

#[check]
#[name = "ModServer"]
async fn mod_server_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    match msg.guild_id {
        Some(n) => {
            if n.0 == config.moderation_server {
                return Ok(());
            }
        }
        None => {}
    }

    match msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.reference_message(msg).embed(|e| {
                e.color(Colour(10038562));
                e.description(
                    "**Error**: You can't use these commands outside the moderation server!",
                )
            })
        })
        .await
    {
        Ok(_) => {}
        Err(e) => error!("failed to send message: {}", e),
    }

    Err(Reason::User(
        "This command is only meant for the moderation server!".to_string(),
    ))
}

#[check]
#[name = "Sudo"]
async fn sudo_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    if config.sudoers.contains(&msg.author.id.0) {
        Ok(())
    } else {
        match msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg).embed(|e| {
                    e.color(Colour(10038562));
                    e.description("**Error**: You, good sir, aren't in the sudoers list")
                })
            })
            .await
        {
            Ok(_) => {}
            Err(e) => error!("failed to send message: {}", e),
        }

        Err(Reason::User("User not a sudoer".to_string()))
    }
}

#[check]
#[name = "Channel"]
async fn channel_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    if config.public_channels.contains(&msg.channel_id.0) {
        match msg.author.id.create_dm_channel(&ctx.http).await {
            Ok(n) => {
                match n
                    .send_message(&ctx.http, |m| {
                        m.content("You can't use commands in public channels!")
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(e) => error!("failed to send message to user: {}", e),
                }
            }
            Err(e) => error!("failed to create DM channel: {}", e),
        }

        Err(Reason::User(
            "Can't execute commands in a public channel!".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    _: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<TypeMapConfig>().unwrap();

    let help_options = HelpOptions {
        names: &["help"],
        suggestion_text: "Did you mean `{}`?",
        no_help_available_text: "**Error**: No help available.",
        usage_label: "Usage",
        usage_sample_label: "Sample usage",
        ungrouped_label: "Ungrouped",
        description_label: "Description",
        grouped_label: "Group",
        aliases_label: "Aliases",
        guild_only_text: "Only in servers",
        checks_label: "Checks",
        sub_commands_label: "Sub Commands",
        dm_only_text: "Only in DM",
        dm_and_guild_text: "In DM and servers",
        available_text: "Available",
        command_not_found_text: "Could not find: `{}`.",
        individual_command_tip:
            "For more information about a specific command, just pass that command as an argument.\n",
        strikethrough_commands_tip_in_dm: Some(
            "~~`Strikethrough commands`~~ are unavailable because you need to be in the right channel.",
        ),
        strikethrough_commands_tip_in_guild: Some(
            "~~`Strikethrough commands`~~ are unavailable because you need to be in the right channel.",
        ),
        group_prefix: "Prefix",
        lacking_role: HelpBehaviour::Nothing,
        lacking_permissions: HelpBehaviour::Hide,
        lacking_ownership: HelpBehaviour::Hide,
        lacking_conditions: HelpBehaviour::Strike,
        wrong_channel: HelpBehaviour::Strike,
        embed_error_colour: Colour(10038562),
        embed_success_colour: Colour(16178136),
        max_levenshtein_distance: 3,
        indention_prefix: "+",
    };

    if !config.public_channels.contains(&msg.channel_id.0) {
        let _ = help_commands::with_embeds(ctx, msg, args, &help_options, groups, owners).await;
    }

    Ok(())
}
