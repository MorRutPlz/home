use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::TypingStartEvent,
        guild::Member,
        id::{ChannelId, GuildId, MessageId, UserId},
        interactions::Interaction,
        prelude::{Activity, OnlineStatus, Ready},
    },
};

use crate::{commands::register_commands, debug, error, info, typemap::TypeMapSharedCache};
use crate::{
    commands::{execute, room::create::create_room},
    typemap::TypeMapConfig,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        register_commands(&ctx).await;

        let data = ctx.data.read().await;
        let config = data.get::<TypeMapConfig>().unwrap();

        let activity = Activity::playing(&config.status);
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;

        info!("Bot started! <3");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.data.is_some() {
            execute(ctx, interaction).await;
        }
    }

    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        if guild_id.0 != 806947535825403904 {
            return;
        }

        {
            let data = ctx.data.read().await;
            let config = data.get::<TypeMapSharedCache>().unwrap();

            match config.get_user_room_map().get(&new_member.user.id) {
                Some(n) => {
                    if n.len() != 0 {
                        return;
                    }
                }
                None => {}
            }
        }

        let mut room_name = new_member
            .user
            .name
            .chars()
            .into_iter()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();

        if room_name.len() > 83 {
            room_name.truncate(83);
        }

        match create_room(&ctx, new_member.user.id, room_name).await {
            Ok(_) => {}
            Err(e) => {
                let data = ctx.data.read().await;
                let config = data.get::<TypeMapConfig>().unwrap();
                let mut sudoers = config.sudoers.iter();

                match new_member.user.create_dm_channel(&ctx.http).await {
                    Ok(n) => match n.send_message(&ctx.http, |m| m.set_embed(e.clone())).await {
                        Ok(_) => {}
                        Err(e) => error!("failed to send message: {}", e),
                    },
                    Err(e) => error!("failed to create DM channel: {}", e),
                }

                loop {
                    match sudoers.next() {
                        Some(n) => match UserId(*n).create_dm_channel(&ctx.http).await {
                            Ok(n) => {
                                match n.send_message(&ctx.http, |m| m.set_embed(e.clone())).await {
                                    Ok(_) => {}
                                    Err(e) => error!("failed to send message: {}", e),
                                }
                            }
                            Err(e) => error!("failed to create DM channel: {}", e),
                        },
                        None => break,
                    }
                }
            }
        }
    }

    async fn typing_start(&self, ctx: Context, e: TypingStartEvent) {
        match e.guild_id {
            Some(n) => {
                let data = ctx.data.read().await;
                let config = data.get::<TypeMapConfig>().unwrap();

                if n.0 == config.main_server {}
            }
            None => {}
        }
    }

    async fn message_delete(
        &self,
        ctx: Context,
        _: ChannelId,
        deleted: MessageId,
        guild: Option<GuildId>,
    ) {
    }
}
