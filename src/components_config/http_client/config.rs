use std::time::Duration;

use crate::message::MsgDataBound;

use super::{request_input::RequestInput, request_periodic::RequestPeriodic};

/// Параметры компонента http-client
#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: MsgDataBound,
{
    /// URL сервера
    ///
    /// *Примеры:*
    ///
    /// ```
    /// base_url: "http://10.0.6.5:80"
    /// ```
    pub base_url: &'static str,
    /// Таймаут запроса
    pub timeout: Duration,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_input: Vec<RequestInput<TMessage>>,
    /// Периодические запросы
    pub requests_periodic: Vec<RequestPeriodic<TMessage>>,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::message::{example_message::*, *};

    use super::super::*;

    #[test]
    fn connect_with_http_server() {
        Config::<Custom> {
            base_url: "http://10.0.6.5:80",
            timeout: Duration::from_secs(5),
            requests_input: vec![RequestInput {
                fn_input: |msg| {
                    let param = HttpParam::Post {
                        endpoint: "/messages".into(),
                        body: msg.serialize().unwrap(),
                    };
                    Some(param)
                },
                on_success: |_| Ok(vec![]),
                on_failure: Vec::new,
            }],
            requests_periodic: vec![RequestPeriodic {
                period: Duration::from_secs(2),
                http_param: HttpParam::Get {
                    endpoint: "/messages".into(),
                },
                on_success: |data| {
                    let msgs = Message::deserialize_many(data)?;
                    Ok(msgs)
                },
                on_failure: Vec::new,
            }],
        };
    }
}
