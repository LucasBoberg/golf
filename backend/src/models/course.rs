use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::schema::courses;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = courses)]
pub struct CourseDTO {
    pub name: String,
    pub red_cr: f32,
    pub yellow_cr: f32,
    pub blue_cr: Option<f32>,
    pub white_cr: Option<f32>,
    pub slope_red: i32,
    pub slope_yellow: i32,
    pub slope_blue: Option<i32>,
    pub slope_white: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = courses)]
pub struct Course {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub name: String,
    pub red_cr: f32,
    pub yellow_cr: f32,
    pub blue_cr: Option<f32>,
    pub white_cr: Option<f32>,
    pub slope_red: i32,
    pub slope_yellow: i32,
    pub slope_blue: Option<i32>,
    pub slope_white: Option<i32>,
}
