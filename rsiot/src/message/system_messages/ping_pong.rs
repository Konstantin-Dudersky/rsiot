use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ping {
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pong {
    pub count: u32,
}
