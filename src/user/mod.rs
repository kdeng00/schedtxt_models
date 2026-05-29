use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
	pub id:         uuid::Uuid,
	pub phone_number: String,
	pub username:   String,
	pub password:   String,
	// Firstname is a pointer
	pub firstname: String,
	// Lastname is a pointer
	pub lastname: String,
    #[serde(with = "time::serde::rfc3339::option")]
	pub created:  Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
	pub last_login: Option<time::OffsetDateTime>,
}

