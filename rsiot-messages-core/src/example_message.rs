//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use crate::{eav, eav_helpers, msg_types, Deserialize, IMessage, Serialize};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExampleMessage {
    ValueInstantF64(msg_types::Value<f64>),
    ValueInstantBool(msg_types::Value<bool>),
    ValueInstantString(msg_types::Value<String>),
    Command(msg_types::Command),
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
            ExampleMessage::Command(msg_content) => eav_helpers::Command {
                ts: msg_content.ts,
                entity: "Command".into(),
                attr: "".into(),
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
        let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(12.3));
        let eav = msg.into_eav();
        println!("{:?}", eav);
    }
}
