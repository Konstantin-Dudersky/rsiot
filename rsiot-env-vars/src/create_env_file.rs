//! Сериализация настроек
//!
//! Используется TOML. Пакет envy умеет только десериализовывать, TOML наиболее близкий.
//! Вручную переводим в UPPER_CASE, у envy проблемы, если использовать атрибуты из serde

use std::fs::write;

use toml::to_string as serialize;

use crate::{Errors, IEnvVars};

/// Создать файл с настройками по-умолчанию
pub fn create_env_file<TEnvVars>(filename: &str) -> Result<(), Errors>
where
    TEnvVars: IEnvVars,
{
    // значения по-умолчанию
    let default = TEnvVars::default();
    // сериализуем в TOML
    let s = serialize(&default)?;
    // переводим название переменных в UPPER_CASE
    let s = lines_to_upper(&s)?;
    write(filename, s)?;
    Ok(())
}

/// Итерируемся по строкам и в каждой строке переводим название переменной в UPPER_CASE
fn lines_to_upper(input: &str) -> Result<String, Errors> {
    let mut output = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(line_to_upper)
        .collect::<Result<Vec<String>, _>>()?;
    output.push("".to_string());
    Ok(output.join("\n"))
}

/// Перевести название переменной в строке в UPPER_CASE
fn line_to_upper(line: &str) -> Result<String, Errors> {
    let parts = line.split(" = ").collect::<Vec<&str>>();
    if parts.len() != 2 {
        let err = format!(
            "Неправильная строка, необходима строка в виде
var <space> = <space> value: {}",
            line
        );
        return Err(Errors::ToUppercase(err));
    };
    let output = format!("{} = {}", parts[0].to_uppercase(), parts[1]);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to_upper_test() {
        let input = "api_ws_port = api_ws_port";
        let expected = "API_WS_PORT = api_ws_port";

        let output = line_to_upper(input).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn lines_to_upper_test() {
        let input = "api_ws_port = api_ws_port
db_user = \"postgres\"";
        let expected = "API_WS_PORT = api_ws_port
DB_USER = \"postgres\"";

        let output = lines_to_upper(input).unwrap();
        assert_eq!(expected, output);
    }
}
