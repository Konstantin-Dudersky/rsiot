use std::fmt::Debug;
use std::time::Duration;

use tracing::warn;

use crate::{
    components_config::http_general::HttpDataBound,
    message::MsgDataBound,
    serde_utils::{SerdeAlg, SerdeAlgKind},
};

use super::{
    msg_request::MsgRequest, FnProcessResponseError, FnProcessResponseSuccess, MsgResponse,
    RequestKind,
};

// ANCHOR: RequestPeriodicConfig
/// Параметры периодического запроса
#[derive(Clone, Debug)]
pub struct RequestPeriodicConfig<TMsg, TServerToClient, TClientToServer> {
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,
    /// Тип HTTP-запроса
    pub request_kind: RequestKind,
    /// Путь к ресурсу
    pub endpoint: String,
    /// Периодичность вызова
    pub period: Duration,
    /// Параметры запроса
    pub request_body: TClientToServer,
    /// Функция обработки корректного ответа
    pub fn_process_response_success: FnProcessResponseSuccess<TMsg, TServerToClient>,
    /// Функция обработки ошибки ответа
    pub fn_process_response_error: FnProcessResponseError<TMsg>,
}
// ANCHOR: RequestPeriodicConfig

/// Трейт для реализации периодических запросов
pub trait RequestPeriodic<TMsg>
where
    Self: Debug + Send + Sync,
{
    /// Создание запроса
    fn create_request(&self) -> Option<MsgRequest>;

    /// Обработка ответа
    fn process_response(&self, msg_response: &MsgResponse) -> Option<Vec<TMsg>>;

    /// Получить периодичность вызова
    fn get_period(&self) -> Duration;

    /// Поддержка клонирования
    fn clone_dyn(&self) -> Box<dyn RequestPeriodic<TMsg>>;
}

impl<TMsg, TServerToClient, TClientToServer> RequestPeriodic<TMsg>
    for RequestPeriodicConfig<TMsg, TServerToClient, TClientToServer>
where
    TMsg: 'static + MsgDataBound,
    TClientToServer: 'static + HttpDataBound,
    TServerToClient: 'static + HttpDataBound,
{
    fn create_request(&self) -> Option<MsgRequest> {
        let serde_alg = SerdeAlg::new(self.serde_alg);
        let body_bytes = serde_alg.serialize(&self.request_body);
        let body_bytes = match body_bytes {
            Ok(v) => v,
            Err(e) => {
                warn!("Error serializing http request: {}", e);
                return None;
            }
        };

        let msg_request = MsgRequest::new(self.request_kind, self.endpoint.clone(), body_bytes);
        Some(msg_request)
    }

    fn process_response(&self, msg_response: &MsgResponse) -> Option<Vec<TMsg>> {
        if msg_response.get_endpoint() != &self.endpoint {
            return None;
        }

        super::process_response::process_response(
            self.serde_alg,
            self.fn_process_response_success,
            self.fn_process_response_error,
            msg_response,
        )
    }

    fn get_period(&self) -> Duration {
        self.period
    }

    fn clone_dyn(&self) -> Box<dyn RequestPeriodic<TMsg>> {
        Box::new(self.clone())
    }
}

impl<TMsg> Clone for Box<dyn RequestPeriodic<TMsg>> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
