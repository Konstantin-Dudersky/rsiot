use dotenvy::dotenv;
use envy::from_env;
use tracing::{error, info};

use crate::{Errors, IEnvVars};

/// Загрузить настройки:
/// - из переменных среды
/// - из файла .env в корне проекта
pub fn load_config<TEnvVars>() -> Result<TEnvVars, Errors>
where
    TEnvVars: IEnvVars,
{
    let vars = _load_config();
    match &vars {
        Ok(value) => {
            info!("Загружен файл с переменными: {:?}", value);
        }
        Err(err) => {
            error!("Ошибка загрузки переменных среды: {:?}", err);
        }
    };
    vars
}

fn _load_config<TEnvVars>() -> Result<TEnvVars, Errors>
where
    TEnvVars: IEnvVars,
{
    // загружаем из файла .env
    dotenv()?;
    // десериализуем в структуру
    let vars = from_env::<TEnvVars>()?;
    Ok(vars)
}
