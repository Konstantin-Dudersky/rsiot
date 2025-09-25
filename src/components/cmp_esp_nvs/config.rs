//! Тестирование документации:
//!
//! Нет тестируется - Exec format error (os error 8)
//!
//! ```bash
//! cargo test components::cmp_storage_esp --features="cmp_storage_esp, single-thread" --target="riscv32imc-esp-espidf";
//! ```

use serde::{Serialize, de::DeserializeOwned};

use crate::message::{Message, MsgDataBound};

// ANCHOR: Config
/// Конфигурация cmp_storage_esp
#[derive(Debug)]
pub struct Config<TMsg, TStorageData>
where
    TMsg: MsgDataBound,
    TStorageData: std::fmt::Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    /// Функция для сохранения информации из входных сообщений в памяти ESP.
    pub fn_input: fn(&TStorageData, &Message<TMsg>) -> Option<TStorageData>,

    /// Функция для выдачи сообщений из сохраненных данных.
    ///
    /// Вызывается один раз, при запуске ESP.
    pub fn_output: fn(&TStorageData) -> Vec<Message<TMsg>>,
}
// ANCHOR: Config

#[cfg(test)]
mod tests {

    use super::Config;

    #[test]
    fn fn_input() {
        use crate::message::{example_message::*, *};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
        struct StorageData {
            pub test_f64: f64,
            pub test_i32: i32,
        }

        let _ = Config {
            fn_input: |data: &StorageData, msg: &Message<Custom>| match msg.data {
                MsgData::Custom(Custom::ValueInstantF64(value)) => Some(StorageData {
                    test_f64: value,
                    ..*data
                }),
                _ => None,
            },
            fn_output: |_| vec![],
        };
    }

    #[test]
    fn fn_output() {
        use crate::message::{example_message::*, *};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
        struct StorageData {
            pub test_f64: f64,
            pub test_i32: i32,
        }

        let _ = Config {
            fn_input: |_, _| None,
            fn_output: |data: &StorageData| {
                vec![Message::new(MsgData::Custom(Custom::ValueInstantF64(
                    data.test_f64,
                )))]
            },
        };
    }
}
