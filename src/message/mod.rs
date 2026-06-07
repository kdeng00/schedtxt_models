pub mod event;
pub mod scheduling;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Message {
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub id: Option<uuid::Uuid>,
    pub content: String,
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub user_id: Option<uuid::Uuid>,
}
