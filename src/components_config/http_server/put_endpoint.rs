use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    message::{Message, MsgDataBound},
    serde_utils::{self, SerdeAlg, SerdeAlgKind},
};

// ANCHOR: PutEndpointConfig
/// Конфигурация отдельной точки PUT
#[derive(Clone, Debug)]
pub struct PutEndpointConfig<TMsg, TData>
where
    TMsg: MsgDataBound,
{
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,

    /// Путь
    ///
    /// Примеры:
    ///
    /// ```rust
    /// path: `/data`
    /// ```
    pub path: &'static str,

    /// Функция создания исходящих сообщений на основе входящих данных
    pub fn_output: fn(data: TData) -> Option<Message<TMsg>>,
}
// ANCHOR: PutEndpointConfig

impl<TMsg, TData> PutEndpoint<TMsg> for PutEndpointConfig<TMsg, TData>
where
    TData: 'static + Clone + Debug + DeserializeOwned + Serialize + Send + Sync,
    TMsg: 'static + MsgDataBound,
{
    fn get_path(&self) -> &str {
        self.path
    }

    fn fn_output(&self, request_body: &[u8]) -> Result<Option<Message<TMsg>>, serde_utils::Error> {
        let serde_alg = SerdeAlg::new(self.serde_alg);
        let data: TData = serde_alg.deserialize(request_body)?;
        let msg = (self.fn_output)(data);
        Ok(msg)
    }

    fn clone_dyn(&self) -> Box<dyn PutEndpoint<TMsg>> {
        Box::new(self.clone())
    }
}

/// Трейт для обеспечения логики работы отдельной точик PUT
///
/// В разных точках хранят данные в разных структурах (поле `data`). Трейт нужен для обработки в
/// массиве
pub trait PutEndpoint<TMsg>
where
    Self: Debug + Send + Sync,
{
    /// Получить путь для роутера
    fn get_path(&self) -> &str;

    /// Создание исходящих сообщений на основе входящих данных
    fn fn_output(&self, request_body: &[u8]) -> Result<Option<Message<TMsg>>, serde_utils::Error>;

    /// Поддержка клонирования
    fn clone_dyn(&self) -> Box<dyn PutEndpoint<TMsg>>;
}

impl<TMsg> Clone for Box<dyn PutEndpoint<TMsg>> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
