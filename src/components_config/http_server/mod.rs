//! Конфигурация HTTP-сервера

mod config;
mod content_types;
mod get_endpoint;
mod get_endpoints_collection;
mod handlers;
mod put_endpoint;
mod put_endpoints_collection;

pub use config::Config;
pub use content_types::ContentType;
pub use get_endpoint::{GetEndpoint, GetEndpointConfig};
pub use get_endpoints_collection::GetEndpointsCollection;
pub use handlers::handler_info;
pub use put_endpoint::{PutEndpoint, PutEndpointConfig};
pub use put_endpoints_collection::PutEndpointsCollection;
