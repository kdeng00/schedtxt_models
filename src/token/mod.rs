use serde::Deserialize;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub user_id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(alias = "iss")]
    pub issued: String,
    #[serde(alias = "exp")]
    #[serde(deserialize_with = "deserialize_i64_from_f64")]
    pub expired: i64,
    #[serde(alias = "iat")]
    #[serde(deserialize_with = "deserialize_i64_from_f64")]
    pub issued_at: i64,
}

fn deserialize_i64_from_f64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = f64::deserialize(deserializer)?;
    // Handle NaN and infinity cases
    if val.is_nan() || val.is_infinite() {
        return Err(serde::de::Error::custom("invalid float value"));
    }
    // Round to nearest integer and convert
    let rounded = val.round();
    // Check if the rounded value can fit in i64
    if rounded < (i64::MIN as f64) || rounded > (i64::MAX as f64) {
        Err(serde::de::Error::custom("float out of i64 range"))
    } else {
        Ok(rounded as i64)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct LoginResult {
    pub user_id: uuid::Uuid,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub issued_at: i64,
}

/// Alias for LoginResult
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

#[derive(Debug, utoipa::ToSchema)]
pub struct TokenResource {
    pub message: String,
    pub issuer: String,
    pub audiences: Vec<String>,
    pub user_id: uuid::Uuid,
}

/// Token type
pub const TOKEN_TYPE: &str = "JWT";

pub struct CreateTokenResult {
    pub access_token: String,
    pub issued: i64,
    pub expires_in: i64,
}

pub fn create_token(
    key: &String,
    token_resource: &TokenResource,
    duration: time::Duration,
) -> Result<CreateTokenResult, josekit::JoseError> {
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type(TOKEN_TYPE);

    let mut payload = josekit::jwt::JwtPayload::new();
    let message = &token_resource.message;
    let issuer = &token_resource.issuer;
    let audiences: &Vec<String> = &token_resource.audiences;
    payload.set_subject(message);
    payload.set_issuer(issuer);
    payload.set_audience(audiences.clone());
    if !token_resource.user_id.is_nil() {
        match payload.set_claim("user_id", Some(serde_json::json!(token_resource.user_id))) {
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

            Ok(CreateTokenResult {
                access_token: josekit::jwt::encode_with_signer(&payload, &header, &signer).unwrap(),
                issued: (expire - time::OffsetDateTime::UNIX_EPOCH).whole_seconds(),
                expires_in: (issued - time::OffsetDateTime::UNIX_EPOCH).whole_seconds(),
            })
        }
        Err(e) => Err(josekit::JoseError::InvalidClaim(e.into())),
    }
}
