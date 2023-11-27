use std::fmt::Debug;

use dotenvy::dotenv;
use envy::from_env;
use serde::de::DeserializeOwned;
use tracing::{error, info};

use crate::{Errors, IConfig};

/// Загрузить настройки:
/// - из переменных среды
/// - из файла .env в корне проекта
pub fn load_config<TConfig>() -> Result<TConfig, Errors>
where
    TConfig: IConfig,
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

fn _load_config<T>() -> Result<T, Errors>
where
    T: DeserializeOwned + Debug,
{
    // загружаем из файла .env
    dotenv()?;
    // десериализуем в структуру
    let vars = from_env::<T>()?;
    Ok(vars)
}
