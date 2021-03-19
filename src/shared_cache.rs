use bson::doc;
use futures::StreamExt;
use mongodb::{bson, Database};
use serenity::model::id::{ChannelId, UserId};
use std::collections::HashMap;

use crate::{error, model::RoomInfo};

pub struct SharedCache {
    database: Database,
    user_room_map: HashMap<UserId, Vec<ChannelId>>,
}

impl SharedCache {
    pub async fn new(database: Database) -> SharedCache {
        let user_room_map = get_rooms(&database).await;

        SharedCache {
            database,
            user_room_map,
        }
    }

    pub async fn add_user_room(&mut self, user_id: UserId, channel: ChannelId) -> Result<(), ()> {
        let mut replace = true;

        match self.user_room_map.get_mut(&user_id) {
            Some(n) => n.push(channel),
            None => {
                self.user_room_map.insert(user_id, vec![channel]);
                replace = false;
            }
        }

        let channels = self
            .user_room_map
            .get(&user_id)
            .unwrap()
            .into_iter()
            .map(|x| x.0 as i64)
            .collect::<Vec<_>>();

        let collection = self.database.collection("rooms");

        match match replace {
            false => collection
                .insert_one(
                    doc! {
                        "a": user_id.0,
                        "b": channels
                    },
                    None,
                )
                .await
                .map(|_| {})
                .map_err(|e| format!("{}", e)),
            true => collection
                .replace_one(
                    doc! {
                        "a": user_id.0
                    },
                    doc! {
                        "a": user_id.0,
                        "b": channels
                    },
                    None,
                )
                .await
                .map(|_| {})
                .map_err(|e| format!("{}", e)),
        } {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("error adding new user to rooms map: {}", e);

                let mut channels = self.user_room_map.remove(&user_id).unwrap();
                channels.pop();

                if channels.len() != 0 {
                    self.user_room_map.insert(user_id, channels);
                }

                Err(())
            }
        }
    }

    pub async fn delete_user_room(&mut self, owner: u64, channel_id: u64) {}

    pub fn get_user_room_map(&self) -> &HashMap<UserId, Vec<ChannelId>> {
        &self.user_room_map
    }
}

async fn get_rooms(database: &Database) -> HashMap<UserId, Vec<ChannelId>> {
    let mut map: HashMap<UserId, Vec<ChannelId>> = HashMap::new();

    database
        .collection("rooms")
        .find(None, None)
        .await
        .unwrap()
        .filter_map(|x| async move {
            match x {
                Ok(n) => match bson::from_document::<RoomInfo>(n) {
                    Ok(n) => Some(n),
                    Err(e) => {
                        error!("failed to parse RoomInfo object: {}", e);
                        None
                    }
                },
                Err(e) => {
                    error!("failed to get document from cursor: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|rooms| drop(map.insert(rooms.owner, rooms.channel_ids)));

    map
}
