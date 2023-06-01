use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::schema::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshDTO {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct SignUpDTO {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub hcp: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct UserDTO {
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub hcp: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            hcp: user.hcp,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub hcp: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
