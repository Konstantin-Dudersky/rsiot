#![allow(clippy::module_inception)]

use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{MsgData, MsgDataBound, Timestamp};

// ANCHOR: Message
/// Сообщение
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message<TMsg> {
    /// Данные
    pub data: MsgData<TMsg>,
    /// Ключ
    pub key: String,
    /// Метка времени
    pub ts: Timestamp,

    cmp_source: Uuid,
}
// ANCHOR: Message

impl<TMsg> Message<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Создать новое сообщение
    pub fn new(data: MsgData<TMsg>) -> Self {
        let key = data.key();
        Self {
            data,
            key,
            ts: Default::default(),
            cmp_source: Uuid::default(),
        }
    }

    /// Создать новое сообщение типа `MsgData::Custom`
    pub fn new_custom(custom_data: TMsg) -> Self {
        let data = MsgData::Custom(custom_data);
        let key = data.key();
        Self {
            data,
            key,
            ts: Default::default(),
            cmp_source: Uuid::default(),
        }
    }

    /// Возвращает данные сообщения, если тип сообщения `MsgData::Custom`
    pub fn get_custom_data(&self) -> Option<TMsg> {
        match &self.data {
            MsgData::System(_) => None,
            MsgData::Custom(data) => Some(data.clone()),
        }
    }

    /// Задать идентификатор компонента, который отправил сообщение
    pub fn set_cmp_source(&mut self, id: &Uuid) {
        self.cmp_source = *id;
    }

    /// Проверить, что сообщение было отправлено из указанного источника
    pub fn check_source(&self, id: &Uuid) -> bool {
        &self.cmp_source == id
    }
}
