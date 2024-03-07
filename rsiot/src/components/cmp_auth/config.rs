use rsiot_messages_core::*;

#[derive(Clone)]
pub struct Config {
    pub secret_key: String,

    /// Хранилище данных доступа
    pub store: ConfigStore,
}

/// Тип хранилища данных доступа
#[derive(Clone)]
pub enum ConfigStore {
    Local(Vec<ConfigStoreItem>),
    Surrealdb,
}

#[derive(Clone)]
pub struct ConfigStoreItem {
    pub login: String,
    pub password: String,
    pub role: AuthPermissions,
}
