use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

#[derive(Clone, Deserialize)]
pub struct RoomInfo {
    #[serde(rename = "a")]
    pub owner: u64,
    #[serde(rename = "b")]
    pub channel_id: u64,
    #[serde(rename = "c")]
    pub role_id: u64,
}

impl Serialize for RoomInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RoomInfo", 3)?;
        state.serialize_field("a", &(self.owner as i64))?;
        state.serialize_field("b", &(self.channel_id as i64))?;
        state.serialize_field("c", &(self.role_id as i64))?;
        state.end()
    }
}
