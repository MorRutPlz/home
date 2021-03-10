use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        id::{ChannelId, GuildId, MessageId},
        prelude::{Activity, OnlineStatus, Ready},
    },
};

use crate::info;
use crate::typemap::TypeMapConfig;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = ctx.data.read().await;
        let config = data.get::<TypeMapConfig>().unwrap();

        let activity = Activity::playing(&config.status);
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;

        info!("Bot started! <3");
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
