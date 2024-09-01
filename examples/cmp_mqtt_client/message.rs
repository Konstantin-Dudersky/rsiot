use serde::{Deserialize, Serialize};

use rsiot::message::{MsgDataBound, ServiceBound};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Counter(i32),
}

impl MsgDataBound for Custom {
    type TService = Services;
}

// services ----------------------------------------------------------------------------------------

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum Services {
    publish,
    subscribe,
}

impl ServiceBound for Services {}
