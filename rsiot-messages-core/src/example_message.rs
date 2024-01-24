//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use crate::{eav, eav_helpers, msg_meta, Deserialize, IMessage, MsgContent, MsgMeta, Serialize};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, MsgMeta, PartialEq, Serialize)]
pub enum ExampleMessage {
    ValueInstantF64(MsgContent<f64>),
    ValueInstantBool(MsgContent<bool>),
    ValueInstantString(MsgContent<String>),
}

impl IMessage for ExampleMessage {
    fn into_eav(self) -> Vec<eav::EavModel> {
        match self {
            ExampleMessage::ValueInstantF64(msg_content) => eav_helpers::ValueInstant {
                ts: msg_content.ts,
                entity: "ValueInstantF64".into(),
                attr: "".into(),
                value: msg_content.value.into(),
            }
            .into(),

            ExampleMessage::ValueInstantBool(msg_content) => eav_helpers::ValueInstant {
                ts: msg_content.ts,
                entity: "ValueInstantBool".into(),
                attr: "".into(),
                value: msg_content.value.into(),
            }
            .into(),

            ExampleMessage::ValueInstantString(msg_content) => eav_helpers::ValueInstant {
                ts: msg_content.ts,
                entity: "ValueInstantString".into(),
                attr: "".into(),
                value: msg_content.value.clone().into(),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let msg = ExampleMessage::ValueInstantF64(MsgContent::new(12.3));
        let eav = msg.into_eav();
        println!("{:?}", eav);
    }
}
