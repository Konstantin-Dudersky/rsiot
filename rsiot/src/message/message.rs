#![allow(clippy::module_inception)]

use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{MsgData, MsgDataBound, MsgTrace, Timestamp};

/// Сообщение
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message<TCustom> {
    /// Данные
    pub data: MsgData<TCustom>,
    /// Ключ
    pub key: String,
    /// Метка времени
    pub ts: Timestamp,
    /// Путь, по котором передавалось сообщение
    pub trace: MsgTrace,
}

impl<TCustom> Message<TCustom>
where
    TCustom: MsgDataBound,
{
    /// Создать новое сообщение
    pub fn new(data: MsgData<TCustom>) -> Self {
        let key = define_key(&data);
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
        }
    }

    /// Создать новое сообщение типа `MsgData::Custom`
    pub fn new_custom(custom_data: TCustom) -> Self {
        let data = MsgData::Custom(custom_data);
        let key = define_key(&data);
        Self {
            data,
            key,
            ts: Default::default(),
            trace: MsgTrace::default(),
        }
    }

    /// Возвращает данные сообщения, если тип сообщения `MsgData::Custom`
    pub fn get_custom_data(&self) -> Option<TCustom> {
        match &self.data {
            MsgData::System(_) => None,
            MsgData::Custom(data) => Some(data.clone()),
        }
    }

    /// Добавить запись пути
    pub fn add_trace_item(&mut self, id: &Uuid, name: &str) {
        self.trace.add_trace_item(*id, name.to_string())
    }

    /// Проверяем, что в трейсе сообщения присутсвует компонент с заданным id.
    ///
    /// Полезно для предотварщения зацикливания сообщений, чтобы данный компонент не обрабатывал
    /// сообщения, которые он же и сгенерировал
    pub fn contains_trace_item(&self, id: &Uuid) -> bool {
        self.trace.contains_trace_item(id)
    }
}

/// Определить ключ сообщения по выводу Debug
fn define_key<TCustom>(data: &MsgData<TCustom>) -> String
where
    TCustom: MsgDataBound,
{
    let full_str = format!("{:?}", data);
    let key = full_str.split('(').collect::<Vec<&str>>();
    // Убираем последний элемент. Если тип unit (), нужно убрать два последних элемента
    let skip = if key[key.len() - 2].is_empty() { 2 } else { 1 };
    key[0..key.len() - skip].join("-")
}
