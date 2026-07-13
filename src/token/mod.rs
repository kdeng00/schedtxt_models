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
        let converted: std::time::SystemTime = (*provided_time).into();
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
    pub token_issued: time::OffsetDateTime,
}

pub fn create_token(
    key: &str,
    token_resource: TokenResource,
    duration: time::Duration,
) -> Result<CreateTokenResult, josekit::JoseError> {
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type(TOKEN_TYPE);

    match init_payload(token_resource, duration) {
        Ok((payload, issued, expire)) => {
            let signer = match josekit::jws::alg::hmac::HmacJwsAlgorithm::Hs256
                .signer_from_bytes(key.as_bytes())
            {
                Ok(signer) => signer,
                Err(err) => {
                    return Err(err);
                }
            };
            let access_token = match josekit::jwt::encode_with_signer(&payload, &header, &signer) {
                Ok(access_token) => access_token,
                Err(err) => {
                    return Err(err);
                }
            };

            Ok(CreateTokenResult {
                access_token,
                issued: issued.unix_timestamp(),
                expires_in: expire.unix_timestamp(),
                token_issued: issued,
            })
        }
        Err(e) => Err(josekit::JoseError::InvalidClaim(e.into())),
    }
}

fn init_payload(
    token_resource: TokenResource,
    duration: time::Duration,
) -> Result<
    (
        josekit::jwt::JwtPayload,
        time::OffsetDateTime,
        time::OffsetDateTime,
    ),
    josekit::JoseError,
> {
    let mut payload = josekit::jwt::JwtPayload::new();
    payload.set_subject(token_resource.message);
    payload.set_issuer(token_resource.issuer);
    payload.set_audience(token_resource.audiences);

    if !token_resource.user_id.is_nil() {
        match payload.set_claim("user_id", Some(serde_json::json!(token_resource.user_id))) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    }

    match init_payload_issued(&mut payload, duration) {
        Ok((issued, expire)) => Ok((payload, issued, expire)),
        Err(err) => Err(err),
    }
}

fn init_payload_issued(
    payload: &mut josekit::jwt::JwtPayload,
    duration: time::Duration,
) -> Result<(time::OffsetDateTime, time::OffsetDateTime), josekit::JoseError> {
    match get_issued() {
        Ok(issued) => {
            let expire = issued + duration;
            let issued_at = match util::time_to_std_time(&issued) {
                Ok(issued_at) => issued_at,
                Err(err) => {
                    return Err(josekit::JoseError::InvalidClaim(err.into()));
                }
            };
            let expires_at = match util::time_to_std_time(&expire) {
                Ok(expires_at) => expires_at,
                Err(err) => {
                    return Err(josekit::JoseError::InvalidClaim(err.into()));
                }
            };

            payload.set_issued_at(&issued_at);
            payload.set_expires_at(&expires_at);
            Ok((issued, expire))
        }
        Err(err) => Err(josekit::JoseError::InvalidClaim(err.into())),
    }
}

#[cfg(test)]
mod tests {
    const TEST_MESSAGE: &str = "Testing for schedtxt";
    const TEST_ISSUER: &str = "schedtxt-test";
    const TEST_AUDIENCE: &str = "area-test";
    const TEST_USER_ID: uuid::Uuid = uuid::uuid!("9ab1c75f-d184-4913-ae99-544b4dcfcb41");
    const TEST_KEY: &str = "8342nhf7ycrt4983q7ryfc93w478ryfc3w9487ryfc342w98i7cy";

    #[test]
    fn test_create_token() {
        let token_resource = super::TokenResource {
            message: String::from(TEST_MESSAGE),
            issuer: String::from(TEST_ISSUER),
            audiences: vec![String::from(TEST_AUDIENCE)],
            user_id: TEST_USER_ID,
        };

        let token_duration = time::Duration::minutes(30);
        let key = String::from(TEST_KEY);

        match super::create_token(&key, token_resource, token_duration) {
            Ok(cst) => match time::OffsetDateTime::from_unix_timestamp(cst.issued) {
                Ok(d_result) => {
                    let they_match = {
                        d_result.year() == cst.token_issued.year()
                            && d_result.month() == cst.token_issued.month()
                            && d_result.day() == cst.token_issued.day()
                            && d_result.hour() == cst.token_issued.hour()
                            && d_result.minute() == cst.token_issued.minute()
                            && d_result.second() == cst.token_issued.second()
                    };
                    assert!(they_match, "Issued times do not match");
                }
                Err(err) => {
                    assert!(false, "Error: {err:?}");
                }
            },
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
