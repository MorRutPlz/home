use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, RoleId, UserId};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct RoomsVec {
    pub room: Vec<RoomInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct RoomInfo {
    pub user_id: Vec<u64>,
    pub channel_id: u64,
    pub role_id: u64,
}

impl Into<HashMap<UserId, (ChannelId, RoleId)>> for RoomsVec {
    fn into(self) -> HashMap<UserId, (ChannelId, RoleId)> {
        let mut items = Vec::new();

        self.room.into_iter().for_each(|x| {
            x.user_id.clone().into_iter().for_each(|id| {
                items.push((UserId(id), (ChannelId(x.channel_id), RoleId(x.role_id))))
            });
        });

        items.into_iter().collect()
    }
}
