use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, ServiceBound, TimeToLive};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Counter(i32),
}

impl MsgDataBound for Custom {
    type TService = Services;
}

impl TimeToLive for Custom {}

// services ----------------------------------------------------------------------------------------

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Services {
    publish,
    subscribe,
}

impl ServiceBound for Services {}
