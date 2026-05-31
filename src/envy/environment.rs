pub async fn get_db_url() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::DB_URL;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_secret_main_key() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::SECRET_MAIN_KEY;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_service_passphrase() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::SERVICE_PASSPHRASE;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_secret_key() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::SECRET_KEY;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_root_directory() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::ROOT_DIRECTORY;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_app_base_api_url() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::TEXTSENDER_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_app_auth_base_api_url() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::TEXTSENDER_AUTH_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_app_env() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::APP_ENV;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_backend_port() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::BACKEND_PORT;
    let value = std::env::var(key).expect(key);
    crate::envy::init_envvar(key, &value)
}

pub async fn get_frontend_url() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::FRONTEND_URL;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_rust_log() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::RUST_LOG;
    let value = std::env::var(key).expect(key);

    crate::envy::init_envvar(key, &value)
}

pub async fn get_allowed_origins() -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::envy::keys::ALLOWED_ORIGINS;
    let value = std::env::var(key).expect(key);

    let mut envvar = crate::envy::init_envvar(key, &value);
    crate::envy::init_delimiter(&mut envvar, ',');

    envvar
}

/// Get environment not specified in the code
pub async fn get_env(environment: &str) -> crate::envy::EnvVar {
    dotenvy::dotenv().ok();
    let my_error = format!("{environment} {}", crate::envy::keys::error::GENERAL_ERROR);
    let value = std::env::var(environment).expect(&my_error);

    crate::envy::init_envvar(environment, &value)
}
