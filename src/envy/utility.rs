/// Take the Environment variable and delimitize it. If the value has a delimiter,
/// extract it into some strings
pub fn delimitize(var: &crate::envy::EnvVar) -> Result<Vec<String>, std::io::Error> {
    if var.has_delimiter {
        Ok(var
            .value
            .split(var.delimiter)
            .map(|c| c.parse::<String>().unwrap())
            .collect())
    } else {
        Err(std::io::Error::other(
            "Environment variable does not have a delimiter",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delimitize() {
        let envvar = crate::envy::EnvVar {
            key: "OCEANS".to_string(),
            value: "Atlantic|Pacific|Indian|Arctic".to_string(),
            has_delimiter: true,
            delimiter: '|',
        };
        let all_values = vec!["Atlantic".to_string(), "Pacific".to_string(), "Indian".to_string(), "Arctic".to_string()];

        match delimitize(&envvar) {
            Ok(values) => {
                assert_eq!(values, all_values, "Failure in delimitizing EnvVar values");
            }
            Err(err) => {
                assert!(false, "Error delimitizing: {err:?}");
            }
        }
    }
}
