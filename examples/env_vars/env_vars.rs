#[cfg(feature = "env_vars")]
#[tokio::main]
async fn main() {
    use serde::{Deserialize, Serialize};
    use url::Url;

    /// Структура со всеми переменными
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Config {
        pub db_host: String,
        pub db_port: u16,
        pub db_user: String,
        pub db_password: String,
    }

    /// Задаем настройки по-умолчанию
    impl Default for Config {
        fn default() -> Self {
            Self {
                db_host: "localhost".into(),
                db_port: 5432,
                db_user: "postgres".into(),
                db_password: "postgres".into(),
            }
        }
    }

    /// Добавляем переменные, которые собираются из заданных в файле
    impl Config {
        /// Подключение к БД с данными
        #[allow(dead_code)]
        pub fn db_data_url(&self) -> Url {
            let url = format!(
                "postgres://{}:{}@{}:{}/db_data",
                self.db_user, self.db_password, self.db_host, self.db_port
            );
            Url::parse(&url).expect("Неправильно заданный адрес БД")
        }
    }

    impl IEnvVars for Config {}

    use rsiot::env_vars::{load_config, IEnvVars};

    tracing_subscriber::fmt().init();
    let config = load_config::<Config>();
    println!("{:#?}", config);
}

#[cfg(not(feature = "env_vars"))]
fn main() {}
