//! Конфигурация HTTP-сервера
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc http_server
//! ```

mod get_endpoint;
mod handlers;
mod put_endpoint;

pub use get_endpoint::{create_get_endpoints_hashmap, GetEndpoint, GetEndpointConfig};
pub use handlers::{handler_get, handler_info, handler_put};
pub use put_endpoint::{create_put_endpoints_hashmap, PutEndpoint, PutEndpointConfig};

use crate::message::*;

/// Конфигурация компонента http-server
#[derive(Clone, Debug)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Конфигурация точек GET
    pub get_endpoints: Vec<Box<dyn GetEndpoint<TMsg>>>,

    /// Конфигурация точек PUT
    pub put_endpoints: Vec<Box<dyn PutEndpoint<TMsg>>>,
}

/// Коллекция точек GET
pub type GetEndpointsHashMap<TMsg> = std::collections::HashMap<String, Box<dyn GetEndpoint<TMsg>>>;
/// Коллекция точек PUT
pub type PutEndpointsHashMap<TMsg> = std::collections::HashMap<String, Box<dyn PutEndpoint<TMsg>>>;

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::message::example_message::*;

    #[allow(clippy::no_effect)]
    #[test]
    fn stub() {
        Config::<Custom> {
            port: 8000,
            get_endpoints: vec![],
            put_endpoints: vec![],
        };
    }

    #[allow(clippy::no_effect)]
    #[test]
    fn fn_input_json() {
        Config::<Custom> {
            port: 8000,
            get_endpoints: vec![],
            put_endpoints: vec![],
        };
    }

    #[allow(clippy::no_effect)]
    #[test]
    fn fn_output_json() {
        Config::<Custom> {
            port: 8000,
            get_endpoints: vec![],
            put_endpoints: vec![],
        };
    }
}
