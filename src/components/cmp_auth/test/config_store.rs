use crate::{components::cmp_auth, message::AuthPermissions};

#[test]
fn test() {
    // Локальное хранилище
    let store_0 = cmp_auth::ConfigStore::Local(vec![cmp_auth::ConfigStoreLocalItem {
        login: "admin".into(),
        password: "admin".into(),
        role: AuthPermissions::Admin,
    }]);

    let _ = cmp_auth::Config {
        store: store_0,
        ..Default::default()
    };
}
