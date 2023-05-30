use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::repository::schema::rounds)]
pub struct RoundDTO {
    pub course_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::repository::schema::rounds)]
pub struct Round {
    #[serde(default)]
    pub id: Uuid,
    pub course_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
