use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: uuid::Uuid,
    pub role: String,
    #[serde(alias = "iss")]
    pub issued: String,
    #[serde(alias = "exp")]
    pub expired: i64,
    #[serde(alias = "iat")]
    pub issued_at: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginResult {
    pub user_id: uuid::Uuid,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub issued_at: i64,
}

// Alias for LoginResult
pub type Login = LoginResult;
