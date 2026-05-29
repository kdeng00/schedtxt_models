use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: uuid::Uuid,
    pub role: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginResult {
    pub user_id: uuid::Uuid,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub issued_at: i64,
}

// Make alias for LoginResult for Login

// Alias for LoginResult
// Will be deprecated at some point
// type Login = LoginResult
type Login = LoginResult;
