use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub id: uuid::Uuid,
    // empty?
    #[serde(skip_serializing_if = "String::is_empty")]
    pub firstname: String,
    // empty?
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lastname: String,
    // empty?
    #[serde(skip_serializing_if = "String::is_empty")]
    pub nickname: String,
    pub phone_number: String,
    // empty
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub user_id: uuid::Uuid,
}
