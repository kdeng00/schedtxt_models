pub const MESSAGE_EVENT_RESPONSE_STATUS_INSTANT: &str = "INSTANT";
pub const MESSAGE_EVENT_RESPONSE_STATUS_SCHEDULED: &str = "SCHEDULED";

#[derive(
    Clone, Debug, Default, serde::Deserialize, serde::Serialize, sqlx::FromRow, utoipa::ToSchema,
)]
pub struct MessageEventResponse {
    pub id: uuid::Uuid,
    pub scheduled_message_event_id: uuid::Uuid,
    /// Stores a json response of the sent message
    pub response: String,
    pub user_id: uuid::Uuid,
    pub contact_id: uuid::Uuid,
    pub message_id: uuid::Uuid,
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub sent: Option<time::OffsetDateTime>,
}
