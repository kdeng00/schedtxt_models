use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Message {
    pub id: uuid::Uuid,
    pub content: String,
    pub user_id: uuid::Uuid,
}
