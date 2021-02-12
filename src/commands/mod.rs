pub mod room;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::{check, help},
        Args, CommandGroup, CommandOptions, CommandResult, HelpBehaviour, HelpOptions, Reason,
    },
    model::{channel::Message, id::UserId},
    prelude::TypeMapKey,
    utils::Colour,
};

use std::collections::HashSet;

use crate::config::Config;

pub struct ConfigTMK;

impl TypeMapKey for ConfigTMK {
    type Value = Config;
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
    let config = data.get::<ConfigTMK>().unwrap();

    if config.public_channels.contains(&msg.channel_id.0) {
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
    let config = data.get::<ConfigTMK>().unwrap();

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
