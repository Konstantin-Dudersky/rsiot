//! Пример реализации сообщения. Можно использовать для тестирования компонентов

use super::{Deserialize, MsgDataBound, Serialize, TimeToLive, TimeToLiveValue};

/// Пример реализации сообщения. Можно использовать для тестирования компонентов
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// ESP - кнопка BOOT
    EspBootButton(bool),
    /// ESP - выход на реле
    EspRelay(bool),
    MotorM1(Motor),
    MotorM2(Motor),
    SaveToFilesystem(u64),
}

impl TimeToLive for Custom {
    fn time_to_live(&self) -> TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataGroup {
    /// Значение типа f64 в структуре
    DataGroupF64(f64),
    /// Вложенная в группу структура
    DataGroupStruct(StructInDataGroup),
}

/// Пример типовой структуры
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(missing_docs)]
pub enum Motor {
    Status1(bool),
    Status2(bool),
    Status3(bool),
    Status4(bool),
}

impl MsgDataBound for Custom {}

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

        let msg = Message::new_custom(Custom::MotorM1(Motor::Status1(false)));
        assert_eq!("Custom-MotorM1-Status1", msg.key);

        let msg = Message::new_custom(Custom::MotorM2(Motor::Status1(false)));
        assert_eq!("Custom-MotorM2-Status1", msg.key);
    }
}
