use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
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
    #[serde(skip)]
    pub salt_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ServiceUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub passphrase: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    // When serializing to json, if empty do not populate
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<time::OffsetDateTime>,
    #[serde(skip)]
    pub salt_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Organization {
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserLoginHistory {
    pub id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub login_time: Option<time::OffsetDateTime>,
    pub user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ServiceUserLoginHistory {
    pub id: uuid::Uuid,
    #[serde(with = "time::serde::rfc3339::option")]
    pub login_time: Option<time::OffsetDateTime>,
    pub service_user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserProfile {
    pub user_id: uuid::Uuid,
    pub phone_number: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Salt {
    #[serde(skip_serializing_if = "crate::init::is_uuid_nil")]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub salt: String,
}

impl Salt {
    pub fn to_json(&self, output_pretty: bool) -> Result<String, serde_json::Error> {
        if output_pretty {
            serde_json::to_string_pretty(&self)
        } else {
            serde_json::to_string(&self)
        }
    }
}

pub fn init_user_profile(usr: &User) -> UserProfile {
    UserProfile {
        user_id: usr.id,
        phone_number: usr.phone_number.clone(),
        username: usr.username.clone(),
        firstname: usr.firstname.clone(),
        lastname: usr.lastname.clone(),
        created: usr.created,
        last_login: usr.last_login,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_user_profile() {
        let usr = super::User {
            id: uuid::Uuid::new_v4(),
            phone_number: String::from("+12308438462"),
            username: String::from("Cowboy99"),
            password: String::from("238n7cyr2q89d3xhe2"),
            firstname: String::from("BillyBob"),
            lastname: String::from("Jones"),
            created: Some(time::OffsetDateTime::now_utc()),
            last_login: Some(time::OffsetDateTime::now_utc()),
            ..Default::default()
        };

        let profile = super::init_user_profile(&usr);
        assert_eq!(
            usr.phone_number, profile.phone_number,
            "Phone numbers are not equal"
        );
    }
}
