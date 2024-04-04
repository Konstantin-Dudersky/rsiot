use crate::message::*;

/// Конфигурация cmp_auth
#[derive(Clone)]
pub struct Config {
    /// Секретный ключ для валидации токенов
    pub secret_key: String,

    /// Хранилище данных доступа
    pub store: ConfigStore,
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
