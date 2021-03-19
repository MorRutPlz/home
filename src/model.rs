use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use serenity::model::id::{ChannelId, UserId};

#[derive(Clone, Deserialize, PartialEq, Eq)]
pub struct RoomInfo {
    #[serde(rename = "a")]
    pub owner: UserId,
    #[serde(rename = "b")]
    pub channel_ids: Vec<ChannelId>,
}

impl Serialize for RoomInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RoomInfo", 3)?;
        state.serialize_field("a", &(self.owner.0 as i64))?;
        state.serialize_field(
            "b",
            &(self
                .channel_ids
                .iter()
                .map(|x| x.0 as i64)
                .collect::<Vec<_>>()),
        )?;
        state.end()
    }
}
