use serde::{Deserialize, Serialize};

use rsiot::message::{example_service::*, MsgDataBound, MsgRoute, TimeToLiveValue};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Message {
    TestMessage(i32),
}

impl MsgDataBound for Message {
    type TService = Service;

    fn define_enabled_routes(&self) -> MsgRoute<Self::TService> {
        MsgRoute::default()
    }

    fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}
