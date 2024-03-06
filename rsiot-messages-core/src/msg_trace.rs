//! Структуры для представления пути сообщения

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Timestamp;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
struct TraceValue {
    id: Uuid,
    ts: Timestamp,
    name: String,
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Serialize)]
pub struct MsgTrace(Vec<TraceValue>);

impl MsgTrace {
    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }

    pub fn insert(&mut self, id: Uuid, name: String) {
        let ts = Timestamp::default();
        let value = TraceValue { ts, name, id };
        self.0.push(value);
    }

    pub fn contains_key(&self, id: &Uuid) -> bool {
        self.0.iter().any(|tv| &tv.id == id)
    }

    /// Возвращает все идентификаторы пути сообщения
    pub fn get_ids(self) -> HashSet<Uuid> {
        self.0.iter().map(|i| i.id).collect()
    }
}
