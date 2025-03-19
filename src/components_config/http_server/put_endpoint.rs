use std::{collections::HashMap, fmt::Debug};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;

use crate::message::{Message, MsgDataBound};

/// Конфигурация отдельной точки PUT
#[derive(Clone, Debug)]
pub struct PutEndpointConfig<TMsg, TData>
where
    TMsg: MsgDataBound,
{
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

impl<TMsg, TData> PutEndpoint<TMsg> for PutEndpointConfig<TMsg, TData>
where
    TData: 'static + Clone + Debug + DeserializeOwned + Serialize + Send + Sync,
    TMsg: 'static + MsgDataBound,
{
    fn get_path(&self) -> &str {
        self.path
    }

    fn fn_output(&self, request_body: &str) -> Result<Option<Message<TMsg>>, serde_json::Error> {
        let data: TData = from_str(request_body)?;
        let msg = (self.fn_output)(data);
        Ok(msg)
    }

    fn clone_dyn(&self) -> Box<dyn PutEndpoint<TMsg>> {
        Box::new(self.clone())
    }
}

/// Создать коллекцию точек PUT на основе конфигурации
pub fn create_put_endpoints_hashmap<TMsg>(
    config_endpoints: &[Box<dyn PutEndpoint<TMsg>>],
) -> HashMap<String, Box<dyn PutEndpoint<TMsg>>>
where
    TMsg: MsgDataBound,
{
    let mut endpoints = HashMap::new();
    for endpoint in config_endpoints {
        endpoints.insert(endpoint.get_path().to_string(), endpoint.clone());
    }
    endpoints
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
    fn fn_output(&self, request_body: &str) -> Result<Option<Message<TMsg>>, serde_json::Error>;

    /// Поддержка клонирования
    fn clone_dyn(&self) -> Box<dyn PutEndpoint<TMsg>>;
}

impl<TMsg> Clone for Box<dyn PutEndpoint<TMsg>> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
