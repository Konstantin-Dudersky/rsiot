use serde::{Deserialize, Serialize};

use super::{system_messages::*, MsgDataBound, TimeToLive, TimeToLiveValue};

/// Тип сообщения
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgData<TCustom> {
    /// Системные сообщения
    System(System),
    /// Пользовательские сообщения
    Custom(TCustom),
}

impl<TCustom> MsgData<TCustom>
where
    TCustom: MsgDataBound,
{
    /// Разрешен ли марштур данного сообщения
    pub fn is_route_enabled(
        &self,
        src: Option<TCustom::TService>,
        dst: Option<TCustom::TService>,
    ) -> bool {
        match &self {
            MsgData::System(_) => true,
            MsgData::Custom(custom) => custom.is_route_enabled(src, dst),
        }
    }
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
