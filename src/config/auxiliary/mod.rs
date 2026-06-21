#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TwilioConfig {
    #[serde(alias = "auth_id")]
    pub account_sid: String,
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

// TODO: Remove the async at a later point
pub async fn load_config() -> Result<TwilioConfig, std::io::Error> {
    let auth_sid_var = crate::envy::environment::get_env("TWILIO_AUTH_SID").await;
    let service_sid_var = crate::envy::environment::get_env("TWILIO_SERVICE_SID").await;
    let auth_token_var = crate::envy::environment::get_env("TWILIO_AUTH_TOKEN").await;
    let phone_number_var = crate::envy::environment::get_env("TWILIO_PHONE_NUMBER").await;

    if auth_sid_var.value.is_empty()
        || service_sid_var.value.is_empty()
        || auth_token_var.value.is_empty()
        || phone_number_var.value.is_empty()
    {
        Err(std::io::Error::other("Config not set"))
    } else {
        Ok(TwilioConfig {
            service_sid: service_sid_var.value,
            auth_token: auth_token_var.value,
            account_sid: auth_sid_var.value,
            phone_number: phone_number_var.value,
        })
    }
}
