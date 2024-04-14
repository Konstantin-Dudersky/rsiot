use crate::message::*;

/// https://konstantin-dudersky.github.io/rsiot-docs/components/cmp_auth.html
#[derive(Clone)]
pub struct Config {
    ///  https://konstantin-dudersky.github.io/rsiot-docs/components/cmp_auth.html#secret_key
    pub secret_key: String,

    /// https://konstantin-dudersky.github.io/rsiot-docs/components/cmp_auth.html#store
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

#[cfg(test)]
mod tests {

    use crate::{components::cmp_auth, message::AuthPermissions};

    #[test]
    fn test1() {
        let _auth_config = cmp_auth::Config {
            secret_key: "secret_key".into(),
            // ANCHOR: store_local
            store: cmp_auth::ConfigStore::Local(vec![cmp_auth::ConfigStoreLocalItem {
                login: "admin".into(),
                password: "admin".into(),
                role: AuthPermissions::Admin,
            }]),
            // ANCHOR_END: store_local
        };
    }
}
