use crate::message::MsgDataBound;

use super::{GetEndpoint, PutEndpoint};

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
