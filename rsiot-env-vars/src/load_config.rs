use dotenvy::dotenv;
use envy::from_env;

use crate::{Errors, IEnvVars};

/// Загрузить настройки
///
/// Сначала делается попытка загрузить переменные только из окружения. Если не получилось, тогда из
/// файла .env.
///
/// Для вывода сообщений используются println, поскольку загрузка переменных выполняется до
/// инициализации tracing-subscriber
pub fn load_config<TEnvVars>() -> Result<TEnvVars, Errors>
where
    TEnvVars: IEnvVars,
{
    println!("Пробуем загрузить переменные из окружения");
    let vars = load_from_env();
    match vars {
        Ok(vars) => {
            println!("Переменные из окружения загружены");
            return Ok(vars);
        }
        Err(err) => {
            println!("Ошибка загрузки переменных из окружения: {err}");
        }
    }

    println!("Пробуем загрузить переменные из файла .env");
    let vars = load_from_file();
    match vars {
        Ok(vars) => {
            println!("Переменные из файла успешно загружены");
            return Ok(vars);
        }
        Err(err) => {
            println!("Ошибка загрузки переменных из файла .env: {err}");
            return Err(err);
        }
    }
}

/// Загружаем переменные из окружения
fn load_from_env<TEnvVars>() -> Result<TEnvVars, Errors>
where
    TEnvVars: IEnvVars,
{
    let vars = from_env::<TEnvVars>()?;
    Ok(vars)
}

/// Загружаем переменные из файла .env
fn load_from_file<TEnvVars>() -> Result<TEnvVars, Errors>
where
    TEnvVars: IEnvVars,
{
    dotenv()?;
    let vars = from_env::<TEnvVars>()?;
    Ok(vars)
}
