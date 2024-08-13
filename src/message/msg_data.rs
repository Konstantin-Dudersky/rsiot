use serde::{Deserialize, Serialize};

use super::{system_messages::*, TimeToLive, TimeToLiveValue};

/// Тип сообщения
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgData<TCustom> {
    /// Системные сообщения
    System(System),
    /// Пользовательские сообщения
    Custom(TCustom),
}

impl<Custom> TimeToLive for MsgData<Custom>
where
    Custom: TimeToLive,
{
    fn time_to_live(&self) -> super::TimeToLiveValue {
        match self {
            MsgData::System(_) => TimeToLiveValue::Infinite,
            MsgData::Custom(msg_data) => msg_data.time_to_live(),
        }
    }
}
