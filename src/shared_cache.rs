use futures::StreamExt;
use mongodb::{bson, Database};
use serenity::model::id::UserId;
use std::collections::HashMap;

use crate::{error, model::RoomInfo, warn};

pub struct SharedCache {
    database: Database,
    user_room_map: HashMap<UserId, Vec<RoomInfo>>,
}

impl SharedCache {
    pub async fn new(database: Database) -> SharedCache {
        let user_room_map = get_rooms(&database).await;

        SharedCache {
            database,
            user_room_map,
        }
    }

    pub async fn update_user_room(&mut self, room_info: RoomInfo) {
        match self
            .database
            .collection("rooms")
            .insert_one(bson::to_document(&room_info).unwrap(), None)
            .await
        {
            Ok(_) => match self.user_room_map.get_mut(&UserId(room_info.owner)) {
                Some(n) => n.push(room_info),
                None => drop(
                    self.user_room_map
                        .insert(UserId(room_info.owner), vec![room_info]),
                ),
            },
            Err(e) => {
                error!("error adding new user to rooms map: {}", e)
            }
        }
    }

    pub async fn delete_user_room(&mut self, owner: u64, channel_id: u64) {
        match self.user_room_map.get_mut(&UserId(owner)) {
            Some(n) => {
                let target = n
                    .iter()
                    .filter(|x| x.channel_id == channel_id)
                    .collect::<Vec<_>>();

                if target.len() > 0 {
                    let index = n.iter().position(|x| x == target[0]).unwrap();

                    match self
                        .database
                        .collection("rooms")
                        .delete_one(bson::to_document(&target[0]).unwrap(), None)
                        .await
                    {
                        Ok(_) => {
                            n.swap_remove(index);

                            if n.len() == 0 {
                                self.user_room_map.remove(&UserId(owner));
                            }
                        }
                        Err(e) => {
                            error!("error deleting room: {}", e)
                        }
                    }
                } else {
                    warn!("attempted to delete a room that doesn't exist (channel ID mismatch)");
                }
            }
            None => warn!("attempted to delete a room that doesn't exist (user ID mismatch)"),
        }
    }

    pub fn get_user_room_map(&self) -> &HashMap<UserId, Vec<RoomInfo>> {
        &self.user_room_map
    }
}

async fn get_rooms(database: &Database) -> HashMap<UserId, Vec<RoomInfo>> {
    let mut map: HashMap<UserId, Vec<RoomInfo>> = HashMap::new();

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
        .for_each(|room| match map.get_mut(&UserId(room.owner)) {
            Some(n) => n.push(room),
            None => drop(map.insert(UserId(room.owner), vec![room])),
        });

    map
}
