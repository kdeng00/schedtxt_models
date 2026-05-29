use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Contact {
    pub id: uuid::Uuid,
    // empty?
    pub firstname: String,
    // empty?
    pub lastname: String,
    // empty?
    pub nickname: String,
    pub phone_number: String,
    // empty
    pub user_id: uuid::Uuid,
}
