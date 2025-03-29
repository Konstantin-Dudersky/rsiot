use std::collections::HashMap;

use crate::{message::MsgDataBound, serde_utils};

use super::GetEndpoint;

/// Коллекция точек GET
pub struct GetEndpointsCollection<TMsg>(HashMap<String, Box<dyn GetEndpoint<TMsg>>>);

impl<TMsg> GetEndpointsCollection<TMsg> {
    /// Создать коллекцию точек GET на основе конфигурации
    pub fn new(config_endpoints: &[Box<dyn GetEndpoint<TMsg>>]) -> Self
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

    /// Обработка GET запроса
    pub fn handler<TError>(
        &self,
        path: &str,
        error_unknown_path: fn(String) -> TError,
        error_serde: fn(serde_utils::Error) -> TError,
    ) -> Result<Vec<u8>, TError> {
        self.0
            .get(path)
            .ok_or_else(|| error_unknown_path(path.to_string()))?
            .serialize()
            .map_err(error_serde)
    }

    /// Информация о точках GET для `/info`
    pub fn info(&self) -> String {
        self.0
            .keys()
            .map(|k| format!("<li>{k}</li>"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Массив всех путей GET
    pub fn all_paths(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    /// Обновление данных на основе входящих сообщений
    pub fn fn_input(&mut self, msg: &TMsg) {
        for endpoint in self.0.values_mut() {
            endpoint.fn_input(msg);
        }
    }
}
