use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::TypingStartEvent,
        id::{ChannelId, GuildId, MessageId},
        interactions::Interaction,
        prelude::{Activity, OnlineStatus, Ready},
    },
};

use crate::{commands::execute, typemap::TypeMapConfig};
use crate::{commands::register_commands, info};

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
