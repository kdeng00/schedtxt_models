pub mod event;
pub mod scheduling;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Message {
    pub id: uuid::Uuid,
    pub content: String,
    pub user_id: uuid::Uuid,
}
