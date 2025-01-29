use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Timestamp;

/// Запись пути
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
struct MsgTraceItem {
    id: Uuid,
    ts: Timestamp,
    // name: String,
}

/// Структура для представления пути, по которому передавалось сообщение
#[derive(Debug, Default, Clone, Deserialize, PartialEq, Serialize)]
pub struct MsgTrace(Vec<MsgTraceItem>);

impl MsgTrace {
    /// Создать новый уникальный идентификатор
    ///
    /// TODO - возможно заменить на что-то более компактное - например snowflake
    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }

    /// Добавить запись пути
    pub fn add_trace_item(&mut self, id: Uuid) {
        let ts = Timestamp::default();
        let value = MsgTraceItem { ts, id };
        self.0.push(value);
    }

    /// Определяет, есть ли в пути запись с заданным id
    pub fn contains_trace_item(&self, id: &Uuid) -> bool {
        self.0.iter().any(|tv| &tv.id == id)
    }

    /// Возвращает все идентификаторы пути сообщения
    pub fn get_ids(self) -> HashSet<Uuid> {
        self.0.iter().map(|i| i.id).collect()
    }
}
