use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, RoleId, UserId};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct RoomsVec {
    pub room: Vec<RoomInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct RoomInfo {
    pub user_id: u64,
    pub channel_id: u64,
    pub role_id: u64,
}

impl Into<HashMap<UserId, (ChannelId, RoleId)>> for RoomsVec {
    fn into(self) -> HashMap<UserId, (ChannelId, RoleId)> {
        self.room
            .into_iter()
            .map(|x| {
                (
                    UserId(x.user_id),
                    (ChannelId(x.channel_id), RoleId(x.role_id)),
                )
            })
            .collect::<HashMap<_, _>>()
    }
}
