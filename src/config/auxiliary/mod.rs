use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TwilioConfig {
    // #[serde(skip_serializing_if = "init::is_uuid_nil")]
    #[serde(alias = "auth_id")]
    pub account_sid: uuid::Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub service_sid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub auth_token: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phone_number: String,
}

impl TwilioConfig {
    pub fn print_config(&self) {
	println!("Twilio config");
	println!("Account SID: {:?}", self.account_sid);
	println!("Service SID: {:?}", self.service_sid);
	println!("Auth Token: {:?}", self.auth_token);
	println!("Number: {:?}", self.phone_number);
    }
}

