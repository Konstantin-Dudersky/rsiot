use serde::{Deserialize, Serialize};
use uuid::{Error, Uuid};

/// Идентификатор сервиса
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceId(Option<Uuid>);

impl ServiceId {
    /// Создает новый уникальный идентификатор
    pub fn new() -> Self {
        Self(Some(Uuid::new_v4()))
    }

    /// Получить идентификатор из строки
    pub fn parse_str(input: &str) -> Result<Self, Error> {
        let uuid = Uuid::parse_str(input)?;
        let field = Some(uuid);
        let instance = Self(field);
        Ok(instance)
    }
}

// TODO - переопределить PartialEq. Если внутренне поле None - выдавать неравенство!
