pub mod component {
    pub use rsiot_component_core::*;
}

pub mod message {
    pub use rsiot_messages_core::*;
}

pub use rsiot_extra_components::*;

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

#[cfg(feature = "websocket-client")]
pub mod cmp_websocket_client {
    pub use rsiot_websocket_client::{
        cmp_websocket_client::create, cmp_websocket_client::Config, Error,
    };
}
