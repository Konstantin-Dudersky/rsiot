use std::collections::HashMap;

use crate::{
    message::{Message, MsgDataBound},
    serde_utils,
};

use super::PutEndpoint;

/// Коллекция точек PUT
pub struct PutEndpointsCollection<TMsg>(HashMap<String, Box<dyn PutEndpoint<TMsg>>>);

impl<TMsg> PutEndpointsCollection<TMsg> {
    /// Создать коллекцию точек PUT на основе конфигурации
    pub fn new(config_endpoints: &[Box<dyn PutEndpoint<TMsg>>]) -> Self
    where
        TMsg: MsgDataBound,
    {
        let mut endpoints = HashMap::new();
        for endpoint in config_endpoints {
            let key = endpoint.get_path().to_string();
            let value = endpoint.clone();
            endpoints.insert(key, value);
        }
        Self(endpoints)
    }

    /// Обработка PUT запроса
    pub fn handler<TError>(
        &self,
        path: &str,
        body: &[u8],
        error_unknown_path: fn(String) -> TError,
        error_serde: fn(serde_utils::Error) -> TError,
    ) -> Result<Option<Message<TMsg>>, TError> {
        self.0
            .get(path)
            .ok_or_else(|| error_unknown_path(path.to_string()))?
            .fn_output(body)
            .map_err(error_serde)
    }

    /// Информация о точках PUT для `/info`
    pub fn info(&self) -> String {
        self.0
            .keys()
            .map(|k| format!("<li>{k}</li>"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Массив всех путей PUT
    pub fn all_paths(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }
}
