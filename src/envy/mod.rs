pub mod environment;
pub mod keys;
pub mod utility;

#[derive(Debug, Default, Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    pub has_delimiter: bool,
    pub delimiter: char,
}

pub fn init_envvar(key: &str, value: &str) -> EnvVar {
    EnvVar {
        key: key.to_string(),
        value: value.to_string(),
        has_delimiter: false,
        ..Default::default()
    }
}

pub fn init_delimiter(envvar: &mut EnvVar, delimiter: char) {
    let mut amount_of_delimiters_found: i32 = 0;

    for v in envvar.value.chars() {
        if v == delimiter {
            amount_of_delimiters_found += 1;
        }
    }

    let has_delimiter = amount_of_delimiters_found >= 1;

    if has_delimiter {
        envvar.has_delimiter = has_delimiter;
        envvar.delimiter = delimiter;
    } else {
        envvar.has_delimiter = has_delimiter;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_delimiter() {
        let delimiter: char = ',';
        let value = format!("red,green,white,black");
        let mut env_var = super::EnvVar {
            key: String::from("COLORS"),
            value: value.clone(),
            ..Default::default()
        };

        super::init_delimiter(&mut env_var, delimiter);
        assert_eq!(value, env_var.value, "Colors do not match");
    }
}
