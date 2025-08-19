use std::fmt::Debug;

use tracing::warn;

use crate::{
    components_config::http_general::HttpDataBound,
    message::MsgDataBound,
    serde_utils::{SerdeAlg, SerdeAlgKind},
};

use super::{
    FnCreateRequest, FnProcessResponseError, FnProcessResponseSuccess, MsgRequest, MsgResponse,
    RequestKind,
};

// ANCHOR: RequestInputConfig
/// Параметры запроса на основе входящего потока сообщений
#[derive(Clone, Debug)]
pub struct RequestInputConfig<TMsg, TServerToClient, TClientToServer> {
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,
    /// Тип HTTP-запроса
    pub request_kind: RequestKind,
    /// Путь к ресурсу
    pub endpoint: String,
    /// Функция выдает параметры запроса, на основе входных сообщений
    pub fn_create_request: FnCreateRequest<TMsg, TClientToServer>,
    /// Функция обработки корректного ответа
    pub fn_process_response_success: FnProcessResponseSuccess<TMsg, TServerToClient>,
    /// Функция обработки ошибки ответа
    pub fn_process_response_error: FnProcessResponseError<TMsg>,
}
// ANCHOR: RequestInputConfig

/// Трейт для реализации запросов на основе входящего потока сообщений
pub trait RequestInput<TMsg>
where
    Self: Debug + Send + Sync,
{
    /// Создание запроса
    fn create_request(&self, msg: &TMsg) -> Option<MsgRequest>;

    /// Обработка ответа
    fn process_response(&self, msg_response: &MsgResponse) -> Option<Vec<TMsg>>;

    /// Поддержка клонирования
    fn clone_dyn(&self) -> Box<dyn RequestInput<TMsg>>;
}

impl<TMsg> Clone for Box<dyn RequestInput<TMsg>> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

impl<TMsg, TServerToClient, TClientToServer> RequestInput<TMsg>
    for RequestInputConfig<TMsg, TServerToClient, TClientToServer>
where
    TMsg: 'static + MsgDataBound,
    TClientToServer: 'static + HttpDataBound,
    TServerToClient: 'static + HttpDataBound,
{
    fn create_request(&self, msg: &TMsg) -> Option<MsgRequest> {
        let body = (self.fn_create_request)(msg)?;

        let serde_alg = SerdeAlg::new(self.serde_alg);
        let body_bytes = serde_alg.serialize(&body);
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

    fn clone_dyn(&self) -> Box<dyn RequestInput<TMsg>> {
        Box::new(self.clone())
    }
}
