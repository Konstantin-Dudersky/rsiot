//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use crate::{Deserialize, MsgDataBound, Serialize};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    ValueInstantF64(f64),
    ValueInstantBool(bool),
    ValueInstantString(String),
    DataUnit(()),
    DataGroup(DataGroup),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructInDataGroup {
    pub struct_field1: bool,
    pub struct_field2: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataGroup {
    DataGroupF64(f64),
    DataGroupStruct(StructInDataGroup),
}

impl MsgDataBound for Custom {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Message;

    #[test]
    fn test1() {
        let _msg = Custom::ValueInstantF64(12.3456);
    }

    #[test]
    fn test_key() {
        let msg = Message::new_custom(Custom::DataUnit(()));
        assert_eq!("Custom-DataUnit", msg.key);

        let msg = Message::new_custom(Custom::ValueInstantF64(0.0));
        assert_eq!("Custom-ValueInstantF64", msg.key);

        let msg = Message::new_custom(Custom::DataGroup(DataGroup::DataGroupF64(0.0)));
        assert_eq!("Custom-DataGroup-DataGroupF64", msg.key);

        let msg = Message::new_custom(Custom::DataGroup(DataGroup::DataGroupStruct(
            StructInDataGroup {
                struct_field1: false,
                struct_field2: 0.0,
            },
        )));

        assert_eq!("Custom-DataGroup-DataGroupStruct", msg.key);
    }
}
