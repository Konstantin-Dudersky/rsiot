//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use crate::{message_v2::MsgDataBound, Deserialize, Serialize};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExampleMessage {
    ValueInstantF64(f64),
    ValueInstantBool(bool),
    ValueInstantString(String),
    DataUnit(()),
    DataGroup(DataGroup1),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructInDataGroup {
    pub struct_field1: bool,
    pub struct_field2: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataGroup1 {
    DataGroupF64(f64),
    DataGroupStruct(StructInDataGroup),
}

impl MsgDataBound for ExampleMessage {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Message;

    #[test]
    fn test1() {
        let _msg = ExampleMessage::ValueInstantF64(12.3456);
    }

    #[test]
    fn test_key() {
        let msg = Message::new(ExampleMessage::DataUnit(()));
        println!("{:?}", msg.data);
        assert_eq!("Custom-DataUnit", msg.key);

        let msg = Message::new(ExampleMessage::ValueInstantF64(0.0));
        assert_eq!("Custom-ValueInstantF64", msg.key);

        let msg = Message::new(ExampleMessage::DataGroup(DataGroup1::DataGroupF64(0.0)));
        assert_eq!("Custom-DataGroup-DataGroupF64", msg.key);

        let msg = Message::new(ExampleMessage::DataGroup(DataGroup1::DataGroupStruct(
            StructInDataGroup {
                struct_field1: false,
                struct_field2: 0.0,
            },
        )));
        assert_eq!("Custom-DataGroup-DataGroupStruct", msg.key);

        let msg = Message::new(ExampleMessage::ValueInstantF64(Default::default()));
    }
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
