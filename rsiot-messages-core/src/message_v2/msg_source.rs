use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsgSourceItem {
    pub name: String,
    pub id: Uuid,
}

/// Идентификатор источника сообщения
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsgSource {
    /// Идентификатор исполнителя
    pub executor: MsgSourceItem,
    
    /// Идентификатор компонента внутри исполнителя
    pub component: Option<MsgSourceItem>,

    /// Идентификатор сессии внутри компонента
    pub session: Option<MsgSourceItem>,
}

impl MsgSource {
    /// Создание идентификатора источника сообщения. Создается внутри исполнителя `CmpExecutor`.
    pub fn new(name: &str, id: Uuid) -> Self {
        Self {
            executor: MsgSourceItem {
                name: name.into(),
                id,
            },
            ..Default::default()
        }
    }

    /// Задать идентификатор компонента
    pub fn set_component(&mut self, name: &str, id: Uuid) {
        self.component = Some(MsgSourceItem {
            name: name.into(),
            id,
        });
    }

    /// Задать идентификатор сессии
    pub fn set_session(&mut self, name: &str, id: Uuid) {
        self.session = Some(MsgSourceItem {
            name: name.into(),
            id,
        });
    }

    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }
}
