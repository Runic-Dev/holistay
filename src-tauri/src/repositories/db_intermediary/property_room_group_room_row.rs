use std::collections::HashSet;
use sqlx::FromRow;
use crate::models::domain::room::Room;
use crate::models::domain::room_group::RoomGroup;

#[derive(Clone, FromRow)]
pub struct PropertyRoomGroupRoomRow {
    pub property_id: String,
    pub property_name: String,
    pub property_description: Option<String>,
    pub property_image: Option<String>,
    pub room_group_id: Option<String>,
    pub room_group_name: Option<String>,
    pub room_group_image: Option<String>,
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub room_image: Option<String>
}


pub trait CanBeConvertedToRoomRows {
    fn rooms_from_rows(&self, room_group_id: &str) -> Vec<Room>;
}

pub trait CanBeConvertedToRoomGroups {
    fn room_groups_from_rows(&self) -> Vec<RoomGroup>;
}

impl CanBeConvertedToRoomGroups for Vec<PropertyRoomGroupRoomRow> {
    fn room_groups_from_rows(&self) -> Vec<RoomGroup> {
        self.into_iter().fold(HashSet::<RoomGroup>::new(), |mut rg_set, row| {
            if let (Some(room_group_id), Some(room_group_name)) = (row.room_group_id.as_ref(), row.room_group_name.as_ref()) {
                let room_group = RoomGroup {
                    id: room_group_id.to_string(),
                    name: room_group_name.to_string(),
                    image: row.room_group_image.clone(),
                    description: String::new(),
                    rooms: self.rooms_from_rows(room_group_id)
                };
                rg_set.insert(room_group);
            }
            rg_set
        }).into_iter().collect()
    }
}

impl CanBeConvertedToRoomRows for Vec<PropertyRoomGroupRoomRow> {
    fn rooms_from_rows(&self, room_group_id: &str) -> Vec<Room> {
        self.iter().filter(|f| f.room_group_id.clone().is_some_and(|x| x == room_group_id)).fold(HashSet::<Room>::new(), |mut room_set, row| {
            if let (Some(room_id), Some(room_name)) = (row.clone().room_id, row.clone().room_name) {
                let room = Room { id: room_id, name: room_name, image: row.room_image.clone() };
                room_set.insert(room);
            }
            room_set
        }).into_iter().collect()
    }
}

