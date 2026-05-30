use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageEventResponse {
	pub id:                     uuid::Uuid,
	pub scheduled_message_event_id: uuid::Uuid, 
    // Json field
	pub response:               String,
	pub user_id:                 uuid::Uuid,
	pub contact_id:              uuid::Uuid,
	pub message_id:              uuid::Uuid,
    pub status:                 String,
    #[serde(with = "time::serde::rfc3339::option")]
	pub sent:                   Option<time::OffsetDateTime>,
}

