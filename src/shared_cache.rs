use futures::StreamExt;
use mongodb::{bson, Database};
use serenity::model::id::UserId;
use std::collections::HashMap;

use crate::{error, model::RoomInfo, warn};

pub struct SharedCache {
    database: Database,
    user_room_map: HashMap<UserId, RoomInfo>,
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
            Ok(_) => drop(
                self.user_room_map
                    .insert(UserId(room_info.owner), room_info),
            ),
            Err(e) => {
                error!("error adding new user to rooms map: {}", e)
            }
        }
    }

    pub async fn delete_user_room(&mut self, owner: u64) {
        match self.user_room_map.remove(&UserId(owner)) {
            Some(n) => {
                match self
                    .database
                    .collection("rooms")
                    .delete_one(bson::to_document(&n).unwrap(), None)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!("error deleting room: {}", e);
                        self.user_room_map.insert(UserId(owner), n);
                    }
                }
            }
            None => warn!("attempted to delete a room that doesn't exist"),
        }
    }

    pub fn get_user_room_map(&self) -> &HashMap<UserId, RoomInfo> {
        &self.user_room_map
    }
}

async fn get_rooms(database: &Database) -> HashMap<UserId, RoomInfo> {
    let mut map = HashMap::new();

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
        .for_each(|room| drop(map.insert(UserId(room.owner), room)));

    map
}
