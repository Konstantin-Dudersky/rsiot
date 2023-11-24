// #![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc = include_str!("../README.md")]

//! ## Флаги `feature`:

#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod component {
    pub use rsiot_component_core::*;
}

pub mod message {
    pub use rsiot_messages_core::*;
}

pub use rsiot_extra_components::*;

#[cfg(feature = "http-server")]
pub use rsiot_http_server::cmp_http_server;

#[cfg(feature = "modbus-client")]
pub use rsiot_modbus_client::cmp_modbus_client;

#[cfg(feature = "redis-publisher")]
pub mod cmp_redis_publisher {
    pub use rsiot_redis_publisher::{
        cmp_redis_publisher::create, cmp_redis_publisher::Config, Error,
    };
}

#[cfg(feature = "redis-subscriber")]
pub mod cmp_redis_subscriber {
    pub use rsiot_redis_subscriber::{
        cmp_redis_subscriber::create, cmp_redis_subscriber::Config, Error,
    };
}

#[cfg(feature = "timescaledb-storing")]
pub use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[cfg(feature = "websocket-client")]
pub mod cmp_websocket_client {
    pub use rsiot_websocket_client::{
        cmp_websocket_client::new, cmp_websocket_client::Config, Error,
    };
}

#[cfg(feature = "websocket-server")]
pub use rsiot_websocket_server::cmp_websocket_server;
