#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ScheduledMessageEvent {
    pub id: uuid::Uuid,
    pub contact_id: uuid::Uuid,
    pub message_id: uuid::Uuid,
    pub scheduled_message_id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ScheduledMessage {
    pub id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub scheduled: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    pub status: String,
    pub user_id: uuid::Uuid,
}

pub const READY: &str = "READY";
pub const PENDING: &str = "PENDING";
pub const PROCESSING: &str = "PROCESSING";
pub const DONE: &str = "DONE";
