use serde::{Deserialize, Serialize};

use super::{system_messages::*, MsgDataBound, MsgKey, TimeToLiveValue};

/// Тип сообщения
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgData<TCustom> {
    /// Системные сообщения
    System(System),
    /// Пользовательские сообщения
    Custom(TCustom),
}

impl<TMsg> MsgData<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Задать ограничение времени жизни сообщения
    pub fn define_time_to_live(&self) -> TimeToLiveValue {
        match &self {
            MsgData::System(_) => TimeToLiveValue::Infinite,
            MsgData::Custom(data) => data.define_time_to_live(),
        }
    }

    /// Получить ключ сообщения
    pub fn key(&self) -> String {
        match &self {
            MsgData::System(system) => format!("System-{}", system.key()),
            MsgData::Custom(custom) => format!("Custom-{}", custom.key()),
        }
    }
}

impl<TCustom> MsgData<TCustom> where TCustom: MsgDataBound {}
