use std::collections::HashMap;

#[derive(Clone)]
pub struct Config {
    pub secret_key: String,

    /// Хранилище данных доступа
    pub store: ConfigStoreKind,
}

/// Тип хранилища данных доступа
#[derive(Clone)]
pub enum ConfigStoreKind {
    Hashmap(HashMap<String, String>),
    Surrealdb,
}
