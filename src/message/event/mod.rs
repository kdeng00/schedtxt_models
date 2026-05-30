use serde::{Deserialize, Serialize};


pub struct MessageEventResponse {
	pub id:                     uuid::Uuid,
	pub scheduledMessageEventId: uuid::Uuid, 
    // Json field
	pub response:               String,
	pub userId:                 uuid::Uuid,
	pub contact_id:              uuid::Uuid,
	pub message_id:              uuid::Uuid,
    pub status:                 string,
    #[serde(with = "time::serde::rfc3339::option")]
	pub sent:                   Option<time::OffsetDateTime>,
}

