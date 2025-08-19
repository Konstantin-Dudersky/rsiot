use std::time::Duration;

use crate::{message::MsgDataBound, serde_utils::SerdeAlgKind};

use super::{request_input::RequestInput, request_periodic::RequestPeriodic};

// ANCHOR: Config
/// Параметры компонента http-client
#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: MsgDataBound,
{
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,

    /// URL сервера
    ///
    /// *Примеры:*
    ///
    /// ```
    /// base_url: "http://10.0.6.5:80".into()
    /// ```
    pub base_url: String,
    /// Таймаут запроса
    pub timeout: Duration,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_input: Vec<Box<dyn RequestInput<TMessage>>>,
    /// Периодические запросы
    pub requests_periodic: Vec<Box<dyn RequestPeriodic<TMessage>>>,
}
// ANCHOR: Config

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{message::example_message::*, serde_utils::SerdeAlgKind};

    use super::super::*;

    #[test]
    fn connect_with_http_server() {
        Config::<Custom> {
            serde_alg: SerdeAlgKind::Json,
            base_url: "http://10.0.6.5:80".into(),
            timeout: Duration::from_secs(5),
            requests_input: vec![Box::new(RequestInputConfig::<Custom, (), ()> {
                serde_alg: SerdeAlgKind::Json,
                request_kind: RequestKind::Post,
                endpoint: "/messages".into(),
                fn_create_request: |_msg| Some(()),
                fn_process_response_success: |_| vec![],
                fn_process_response_error: Vec::new,
            })],
            requests_periodic: vec![Box::new(RequestPeriodicConfig::<Custom, (), ()> {
                serde_alg: SerdeAlgKind::Json,
                request_kind: RequestKind::Get,
                endpoint: "/messages".into(),
                period: Duration::from_secs(2),
                request_body: (),
                fn_process_response_success: |_| vec![],
                fn_process_response_error: Vec::new,
            })],
        };
    }
}
