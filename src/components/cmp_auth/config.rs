use crate::message::*;

/// Конфигурация
#[derive(Clone)]
pub struct Config {
    /// Секретный ключ для валидации токенов
    pub secret_key: String,

    /// Хранилище данных доступа
    ///
    /// **Примеры**
    ///
    /// ```no-run
    #[doc = include_str!("./test/config_store.rs")]
    /// ```
    pub store: ConfigStore,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            secret_key: Default::default(),
            store: ConfigStore::Local(vec![]),
        }
    }
}

/// Тип хранилища данных доступа
#[derive(Clone)]
pub enum ConfigStore {
    /// Локальное сохранение - в коде
    Local(Vec<ConfigStoreLocalItem>),

    /// В базе данных SurrealDB
    Surrealdb,
}

/// Запись данных авторизации для одного пользователя
#[derive(Clone)]
pub struct ConfigStoreLocalItem {
    /// Логин
    pub login: String,

    /// Пароль
    pub password: String,

    /// Роль
    pub role: AuthPermissions,
}
