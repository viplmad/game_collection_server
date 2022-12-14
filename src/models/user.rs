use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
}

impl Default for UserDTO {
    fn default() -> Self {
        Self {
            id: -1,
            username: String::default(),
            added_datetime: NaiveDateTime::default(),
            updated_datetime: NaiveDateTime::default(),
        }
    }
}

impl Merge<NewUserDTO> for UserDTO {
    fn merge(self, other: NewUserDTO) -> Self {
        // More useful if more properties are added
        Self {
            id: self.id,
            username: other.username,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for UserDTO {
    const MODEL_NAME: &'static str = "User";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["username"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewUserDTO {
    pub username: String,
}

#[derive(Deserialize, ToSchema)]
pub struct NewPasswordDTO {
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PasswordChangeDTO {
    pub current_password: String,
    pub new_password: String,
}
