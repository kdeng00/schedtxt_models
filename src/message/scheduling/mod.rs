use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ScheduledMessageEvent {
    id: uuid::Uuid,
    contact_id: uuid::Uuid,
    message_id: uuid::Uuid,
    scheduled_message_id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    created: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ScheduledMessage {
    id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    scheduled: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    created: Option<time::OffsetDateTime>,
    status: String,
    user_id: uuid::Uuid,
}

pub const READY: &str = "READY";
pub const PENDING: &str = "PENDING";
pub const PROCESSING: &str = "PROCESSING";
pub const DONE: &str = "DONE";
