use serde::{Deserialize, Serialize};

use super::Timestamp;

/// Тип "Команда"
///
/// Содержит только метку времени
#[derive(Serialize, Clone, Default, Deserialize, Debug, PartialEq)]
pub struct Command {
    pub ts: Timestamp,
}

impl Command {
    pub fn new(ts: Option<Timestamp>) -> Self {
        let ts = match ts {
            Some(value) => value,
            None => Timestamp::default(),
        };
        Self { ts }
    }
}
