#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct Contact {
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub id: Option<uuid::Uuid>,
    // empty?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    // empty?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    // empty?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub phone_number: String,
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub user_id: Option<uuid::Uuid>,
}
