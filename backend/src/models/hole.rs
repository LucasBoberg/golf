use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::schema::holes;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = holes)]
pub struct HoleDTO {
    pub course_id: Uuid,
    pub par: i32,
    pub distance_red: i32,
    pub distance_yellow: i32,
    pub distance_blue: Option<i32>,
    pub distance_white: Option<i32>,
    pub number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = holes)]
pub struct Hole {
    #[serde(default)]
    pub id: Uuid,
    pub course_id: Uuid,
    pub number: i32,
    pub par: i32,
    pub distance_red: i32,
    pub distance_yellow: i32,
    pub distance_blue: Option<i32>,
    pub distance_white: Option<i32>,
}
