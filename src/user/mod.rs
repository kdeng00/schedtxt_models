use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub phone_number: String,
    pub username: String,
    pub password: String,
    // Firstname is a pointer
    pub firstname: String,
    // Lastname is a pointer
    pub lastname: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServiceUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub passphrase: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    // When serializing to json, if empty do not populate
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserLoginHistory {
    pub id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub login_time: Option<time::OffsetDateTime>,
    pub user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServiceUserLoginHistory {
    pub id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub login_time: Option<time::OffsetDateTime>,
    pub service_user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserProfile {
    pub user_id: uuid::Uuid,
    pub phone_number: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub created: Option<time::OffsetDateTime>,
    pub last_login: Option<time::OffsetDateTime>,
}

pub fn init_user_profile(usr: &User) -> UserProfile {
    let usr_profile = UserProfile {
    user_id: usr.id.clone(),
    phone_number: usr.phone_number.clone(),
    username: usr.username.clone(),
    firstname: usr.firstname.clone(),
    lastname: usr.lastname.clone(),
    created: usr.created.clone(),
    last_login: usr.last_login.clone(),
    };

    return usr_profile;
}
