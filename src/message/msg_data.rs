use serde::{Deserialize, Serialize};

use super::{MsgDataBound, MsgKey, system_messages::*};

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
    /// Получить ключ сообщения
    pub fn key(&self) -> String {
        match &self {
            MsgData::System(system) => format!("System-{}", system.key()),
            MsgData::Custom(custom) => format!("Custom-{}", custom.key()),
        }
    }
}

impl<TCustom> MsgData<TCustom> where TCustom: MsgDataBound {}
