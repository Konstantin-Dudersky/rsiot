use serde::{Deserialize, Serialize};

use rsiot::message::{example_service::*, *};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Input4State(bool),
    SetOutput2(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;

    fn define_enabled_routes(&self) -> MsgRoute<Self::TService> {
        MsgRoute::default()
    }

    fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}
