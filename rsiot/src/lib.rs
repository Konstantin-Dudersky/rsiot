#![doc = include_str!("../README.md")]

//! ## Флаги `feature`:

#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod message {
    pub use rsiot_messages_core::*;
}

#[cfg(feature = "components")]
pub mod component {
    pub use rsiot_component_core::*;
    pub use rsiot_extra_components::*;
}

/// Реэкспорт необходимых модулей
pub mod reexport {
    pub use chrono;
    pub use url;

    #[cfg(feature = "components")]
    pub use tokio;
}

#[cfg(feature = "http-client")]
pub use rsiot_http_client::cmp_http_client;

#[cfg(feature = "http-server")]
pub use rsiot_http_server::cmp_http_server;

#[cfg(feature = "modbus-client")]
pub use rsiot_modbus_client::cmp_modbus_client;

#[cfg(feature = "redis-publisher")]
pub use rsiot_redis_publisher::cmp_redis_publisher;

#[cfg(feature = "redis-subscriber")]
pub use rsiot_redis_subscriber::cmp_redis_subscriber;

#[cfg(feature = "timescaledb-storing")]
pub use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[cfg(feature = "websocket-client")]
pub use rsiot_websocket_client::cmp_websocket_client;

#[cfg(feature = "websocket-server")]
pub use rsiot_websocket_server::cmp_websocket_server;
