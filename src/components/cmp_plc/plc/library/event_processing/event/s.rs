use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {
    /// Состояние сообщения
    pub state: State,
}

/// Состояние сообщения
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum State {
    /// Неактивно, квитировано
    #[default]
    NoAct_Ack,
    /// Неактивно, неквитировано
    NoAct_NoAck,
    /// Активно, квитировано
    Act_Ack,
    /// Активно, неквитировано
    Act_NoAck,
}
