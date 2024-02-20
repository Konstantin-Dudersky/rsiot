//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use crate::{Deserialize, Serialize};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExampleMessage {
    ValueInstantF64(f64),
    ValueInstantBool(bool),
    ValueInstantString(String),
}

// impl IMessage for ExampleMessage {
//     fn into_eav(self) -> Vec<eav::EavModel> {
//         let entity = self.key();
//         match self {
//             ExampleMessage::ValueInstantF64(msg_content) => eav_helpers::ValueInstant {
//                 ts: msg_content.ts,
//                 entity,
//                 attr: None,
//                 value: msg_content.value.into(),
//             }
//             .into(),

//             ExampleMessage::ValueInstantBool(msg_content) => eav_helpers::ValueInstant {
//                 ts: msg_content.ts,
//                 entity,
//                 attr: None,
//                 value: msg_content.value.into(),
//             }
//             .into(),

//             ExampleMessage::ValueInstantString(msg_content) => eav_helpers::ValueInstant {
//                 ts: msg_content.ts,
//                 entity,
//                 attr: None,
//                 value: msg_content.value.clone().into(),
//             }
//             .into(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _msg = ExampleMessage::ValueInstantF64(12.3456);
    }
}
