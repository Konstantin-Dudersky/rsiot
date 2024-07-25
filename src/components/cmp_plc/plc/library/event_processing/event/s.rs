use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub state: State,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum State {
    #[default]
    NoAct_Ack,
    NoAct_NoAck,
    Act_Ack,
    Act_NoAck,
}
