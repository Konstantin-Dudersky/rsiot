//! Структура хранения данных точки GET

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    message::MsgDataBound,
    serde_utils::{self, SerdeAlg, SerdeAlgKind},
};

/// Конфигурация отдельной точки GET
#[derive(Clone, Debug)]
pub struct GetEndpointConfig<TMsg, TData> {
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

    /// Данные для точки GET
    ///
    /// На входной структуре необходимо реализовать:
    ///
    /// ```rust
    /// #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    /// ```
    pub data: TData,

    /// Функция обновления данных на основе входящих сообщений
    pub fn_input: fn(&TMsg, &mut TData),
}

impl<TMsg, TData> GetEndpoint<TMsg> for GetEndpointConfig<TMsg, TData>
where
    TMsg: 'static + MsgDataBound,
    TData: 'static + Clone + Debug + DeserializeOwned + Serialize + Send + Sync,
{
    fn get_path(&self) -> &str {
        self.path
    }

    fn fn_input(&mut self, msg: &TMsg) {
        (self.fn_input)(msg, &mut self.data)
    }

    fn serialize(&self) -> Result<Vec<u8>, serde_utils::Error> {
        let serde_alg = SerdeAlg::new(self.serde_alg);
        serde_alg.serialize(&self.data)
    }

    fn clone_dyn(&self) -> Box<dyn GetEndpoint<TMsg>> {
        Box::new(self.clone())
    }
}

/// Трейт для обеспечения логики работы отдельной точик GET
///
/// В разных точках хранят данные в разных структурах (поле `data`). Трейт нужен для обработки в
/// массиве
pub trait GetEndpoint<TMsg>
where
    Self: Debug + Send + Sync,
{
    /// Получить путь для роутера
    fn get_path(&self) -> &str;

    /// Сериализация данных
    fn serialize(&self) -> Result<Vec<u8>, serde_utils::Error>;

    /// Обновление данных на основе входящих сообщений
    fn fn_input(&mut self, msg: &TMsg);

    /// Поддержка клонирования
    fn clone_dyn(&self) -> Box<dyn GetEndpoint<TMsg>>;
}

impl<TMsg> Clone for Box<dyn GetEndpoint<TMsg>> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::message::example_message::Custom;

    #[test]
    fn test1() {
        let mut vec_trait: Vec<Box<dyn GetEndpoint<Custom>>> = vec![];

        #[derive(Clone, Debug, Deserialize, Serialize)]
        struct Data1 {}

        #[derive(Clone, Debug, Deserialize, Serialize)]
        struct Data2 {}

        let end1 = GetEndpointConfig {
            serde_alg: SerdeAlgKind::Json,
            path: "/1",
            data: Data1 {},
            fn_input: |_, _| (),
        };
        let end2 = GetEndpointConfig {
            serde_alg: SerdeAlgKind::Json,
            path: "/2",
            data: Data2 {},
            fn_input: |_, _| (),
        };
        vec_trait.push(Box::new(end1));
        vec_trait.push(Box::new(end2));

        // Собираем в словарь
        let mut map = HashMap::new();

        for e in vec_trait.into_iter() {
            let endpoint = e.get_path().to_string();
            map.insert(endpoint, e);
        }
    }
}
