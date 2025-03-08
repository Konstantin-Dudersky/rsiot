//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use super::{
    example_service::Service, Deserialize, MsgDataBound, MsgKey, MsgRoute, Serialize,
    TimeToLiveValue,
};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgKey)]
#[allow(missing_docs)]
pub enum Custom {
    /// Мгновенное значение типа f64
    ValueInstantF64(f64),
    /// Мгновенное значение типа bool
    ValueInstantBool(bool),
    /// Мгновенное значение типа String
    ValueInstantString(String),
    /// Значение типа unit
    DataUnit(()),
    /// Вложенная группа
    DataGroup(DataGroup),
    /// Вложенный кортеж
    Tuple((String, (bool, bool))),
    /// ValueStruct
    ValueStruct {
        a: f64,
    },
    /// ESP - кнопка BOOT
    EspBootButton(bool),
    /// ESP - выход на реле
    EspRelay(bool),
    SaveToFilesystem(u64),
    // EmptyEnum(EmptyEnum),
}

/// Пример структуры
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructInDataGroup {
    /// Поле 1
    pub struct_field1: bool,
    /// Поле 2
    pub struct_field2: f64,
}

/// Вложенная группа
#[allow(missing_docs)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgKey)]
pub enum DataGroup {
    /// Значение типа f64 в структуре
    DataGroupF64(f64),
    /// Вложенная в группу структура
    DataGroupStruct(StructInDataGroup),
    DataGroupVectorBool(Vec<bool>),
    DataGroupVectorTuple(Vec<(bool, String)>),
}

/// Пустое перечисление
///
/// TODO - появляется почему-то ошибка, хотя cargo-expand показывает корректный код
// #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
// pub enum EmptyEnum {}

impl MsgDataBound for Custom {
    type TService = Service;

    fn define_enabled_routes(&self) -> MsgRoute<Self::TService> {
        MsgRoute::default()
    }

    fn define_time_to_live(&self) -> TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}

#[cfg(test)]
mod tests {
    use super::super::Message;
    use super::*;

    #[test]
    fn test1() {
        let _msg = Custom::ValueInstantF64(12.3456);
    }

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target="x86_64-unknown-linux-gnu" -- message::example_message::tests
    /// ```
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
