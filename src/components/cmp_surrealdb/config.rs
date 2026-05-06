use surrealdb::IndexedResults;

/// Конфигурация cmp_surrealdb
#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    /// Конфигурация подключения к базе данных
    pub connection: ConfigConnection,

    /// root
    pub user: String,

    /// root
    pub password: String,

    /// rsiot
    pub namespace: String,

    /// rsiot
    pub database: String,

    /// Скрипт для инициализации БД. Выполняется, если при запуске не существует namespace
    pub init_script: String,

    /// Конфигурация запросов на основе входных сообщений
    pub request_input: Vec<RequestInputConfig<TMsg>>,

    /// Конфигурация запросов, выполняющихся при запуске
    pub request_start: Vec<RequestStartConfig<TMsg>>,
}

pub type FnOnSuccess<TMsg> = fn(&mut IndexedResults) -> Result<Vec<TMsg>, anyhow::Error>;
pub type FnOnFailure<TMsg> = fn() -> Vec<TMsg>;

/// Конфигурация запросов, которые выполняются на основе входного потока сообщений
#[derive(Clone, Debug)]
pub struct RequestInputConfig<TMsg> {
    /// Функция формирования запроса на основе потока сообщений
    pub fn_input: fn(&TMsg) -> Option<String>,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMsg>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMsg>,
}

/// Конфигурация запросов, которые выполняются один раз при запуске
#[derive(Clone, Debug)]
pub struct RequestStartConfig<TMsg> {
    /// Функция формирования запроса на основе потока сообщений
    pub query: String,
    /// Функция вызывается при успешно выполненном запросе
    pub fn_on_success: FnOnSuccess<TMsg>,
    /// Функция вызывается при ошибке выполнения запроса
    pub fn_on_failure: FnOnFailure<TMsg>,
}

/// Конфигурация подключения к базе данных
#[derive(Clone, Debug)]
pub enum ConfigConnection {
    /// Подключение к базе данных через WebSocket
    ///
    /// Необходимо активировать feature `protocol-http`
    Http {
        /// localhost
        host: String,

        /// 8000
        port: u16,
    },

    /// Подключение к базе данных через WebSocket с TLS
    ///
    /// Необходимо активировать feature `protocol-http`
    HttpSecure {
        /// localhost
        host: String,

        /// 8000
        port: u16,
    },

    /// Подключение к базе данных через Indxdb
    ///
    /// Необходимо активировать feature `kv-indxdb`
    Indxdb {
        /// Название файла базы данных
        file_name: String,
    },

    /// Сохранение данных в памяти
    ///
    /// Необходимо активировать feature `kv-mem`
    Memory,

    /// Сохранение данных в памяти
    ///
    /// Необходимо активировать feature `kv-mem`
    MemoryPersistent {
        /// Название файла базы данных
        file_name: String,
    },

    /// Сохранение данных в файле на диске RocksDB
    ///
    /// Необходимо активировать feature `kv-rocksdb`
    RocksDB {
        /// Название файла базы данных
        ///
        /// Примеры:
        ///
        /// ```
        /// file_name: "database".into()
        /// ```
        file_name: String,
    },

    /// Сохранение данных в файле на диске SurrealKV
    ///
    /// Необходимо активировать feature `kv-surrealkv`
    SurrealKv {
        /// Название файла базы данных
        ///
        /// Примеры:
        ///
        /// ```
        /// file_name: "database".into()
        /// ```
        file_name: String,
    },

    /// Подключение к базе данных TiKV
    ///
    /// Необходимо активировать feature `kv-tikv`
    TiKV {
        /// localhost
        host: String,

        /// 2379
        port: u16,
    },

    /// Подключение к базе данных через WebSocket
    ///
    /// Необходимо активировать feature `protocol-ws`
    Websocket {
        /// localhost
        host: String,

        /// 8000
        port: u16,
    },

    /// Подключение к базе данных через WebSocket
    ///
    /// Необходимо активировать feature `protocol-ws`
    WebsocketSecure {
        /// localhost
        host: String,

        /// 8000
        port: u16,
    },
}

impl ConfigConnection {
    /// Возвращает строку с адресом подключения к базе данных
    pub fn address(&self) -> String {
        match self {
            ConfigConnection::Http { host, port } => format!("http://{}:{}", host, port),
            ConfigConnection::HttpSecure { host, port } => format!("https://{}:{}", host, port),
            ConfigConnection::Indxdb { file_name } => format!("indxdb://{}", file_name),
            ConfigConnection::Memory => "mem://".to_string(),
            ConfigConnection::MemoryPersistent { file_name } => format!("mem://{}", file_name),
            ConfigConnection::RocksDB { file_name } => format!("rocksdb://{}", file_name),
            ConfigConnection::SurrealKv { file_name } => format!("surrealkv://{}", file_name),
            ConfigConnection::TiKV { host, port } => format!("tikv://{}:{}", host, port),
            ConfigConnection::Websocket { host, port } => format!("ws://{}:{}", host, port),
            ConfigConnection::WebsocketSecure { host, port } => format!("wss://{}:{}", host, port),
        }
    }
}
