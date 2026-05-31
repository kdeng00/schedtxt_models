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



pub fn get_issued() -> time::Result<time::OffsetDateTime> {
    Ok(time::OffsetDateTime::now_utc())
}

mod util {
    pub fn time_to_std_time(
        provided_time: &time::OffsetDateTime,
    ) -> Result<std::time::SystemTime, std::time::SystemTimeError> {
        let converted = std::time::SystemTime::from(*provided_time);
        Ok(converted)
    }
}

#[derive(Debug)]
pub struct TokenResource {
    pub message: String,
    pub issuer: String,
    pub audiences: Vec<String>,
    pub id: uuid::Uuid,
}

pub const TOKEN_TYPE: &str = "JWT";

pub fn create_token(
    key: &String,
    token_resource: &TokenResource,
    duration: time::Duration,
) -> Result<(String, i64), josekit::JoseError> {
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type(TOKEN_TYPE);

    let mut payload = josekit::jwt::JwtPayload::new();
    let message = &token_resource.message;
    let issuer = &token_resource.issuer;
    let audiences: &Vec<String> = &token_resource.audiences;
    payload.set_subject(message);
    payload.set_issuer(issuer);
    payload.set_audience(audiences.clone());
    if !token_resource.id.is_nil() {
        match payload.set_claim("id", Some(serde_json::json!(token_resource.id))) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }
    match get_issued() {
        Ok(issued) => {
            let expire = issued + duration;
            payload.set_issued_at(&util::time_to_std_time(&issued).unwrap());
            payload.set_expires_at(&util::time_to_std_time(&expire).unwrap());

            let signer = josekit::jws::alg::hmac::HmacJwsAlgorithm::Hs256
                .signer_from_bytes(key.as_bytes())
                .unwrap();
            Ok((
                josekit::jwt::encode_with_signer(&payload, &header, &signer).unwrap(),
                (expire - time::OffsetDateTime::UNIX_EPOCH).whole_seconds(),
            ))
        }
        Err(e) => Err(josekit::JoseError::InvalidClaim(e.into())),
    }
}
