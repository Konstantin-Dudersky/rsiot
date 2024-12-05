use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::serde_utils::postcard_serde;

/// Сообщение для передачи по UART
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UartMessage<T> {
    /// Адрес устройства
    pub address: u8,
    /// Данные для передачи
    pub payload: T,
}

impl<T> UartMessage<T>
where
    T: Debug + DeserializeOwned + Serialize,
{
    /// Сериализация сообщения
    pub fn serialize<const MESSAGE_LEN: usize>(
        &self,
    ) -> Result<[u8; MESSAGE_LEN], postcard_serde::Error> {
        postcard_serde::serialize(self)
    }

    /// Десериализация сообщения
    pub fn deserialize(data: &mut [u8]) -> Result<Self, postcard_serde::Error> {
        postcard_serde::deserialize(data)
    }
}

#[cfg(test)]
mod tests {
    // Для запуска:
    //
    // ```rust
    // cargo test --target="x86_64-unknown-linux-gnu" --features "cmp_linux_serial_master"
    //  -- components_config::uart_general::uart_message::tests::test1
    // ```

    use super::*;

    #[test]
    fn test1() {
        let msg = UartMessage {
            address: 1,
            payload: 123,
        };

        let mut ser = msg.serialize::<32>().unwrap();

        let _deser: UartMessage<i32> = UartMessage::deserialize(&mut ser).unwrap();
    }
}
