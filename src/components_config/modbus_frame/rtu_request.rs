use bytes::{BufMut, Bytes, BytesMut};

use super::CRC_ALG;

/// Функциональный код запроса
pub enum FunctionCode {
    /// Чтение значений из регистров флагов
    ReadCoils,

    /// Чтение значений из регистров дискретных входов
    ReadDiscreteInputs,

    /// Чтение значений из регистров хранения
    ReadHoldingRegisters {
        /// Количество регистров для чтения
        quantity: u16,
    },

    /// Чтение значений из регистров ввода
    ReadInputRegisters,

    /// Запись значения одного флага
    WriteSingleCoil,

    /// Запись значения в один регистр хранения
    WriteSingleRegister {
        /// Значение
        value: u16,
    },

    /// Запись значений в несколько регистров флагов
    WriteMultipleCoils,

    /// Запись значений в несколько регистров хранения
    WriteMultipleRegisters,
}

/// Создание запроса Modbus RTU
pub struct RTURequest {
    /// Адрес устройства
    pub address: u8,

    /// Стартовый адрес регистра
    pub start_address: u16,

    /// Функциональный код запроса
    pub function_code: FunctionCode,
}

impl RTURequest {
    /// Создать запрос
    pub fn create(self) -> Bytes {
        let mut buf = BytesMut::new();

        buf.put_u8(self.address);

        match self.function_code {
            FunctionCode::ReadCoils => todo!(),
            FunctionCode::ReadDiscreteInputs => todo!(),
            FunctionCode::ReadHoldingRegisters { quantity } => {
                buf.put_u8(0x03);
                buf.put_u16(self.start_address);
                buf.put_u16(quantity);
            }
            FunctionCode::ReadInputRegisters => todo!(),
            FunctionCode::WriteSingleCoil => todo!(),
            FunctionCode::WriteSingleRegister { value } => {
                buf.put_u8(0x06);
                buf.put_u16(self.start_address);
                buf.put_u16(value);
            }
            FunctionCode::WriteMultipleCoils => todo!(),
            FunctionCode::WriteMultipleRegisters => todo!(),
        };

        let checksum = CRC_ALG.checksum(&buf);
        buf.put_u16_le(checksum);

        buf.into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_holding_registers() {
        let test = RTURequest {
            address: 0x01,
            start_address: 2000,
            function_code: FunctionCode::ReadHoldingRegisters { quantity: 13 },
        }
        .create();

        let correct = Bytes::from_static(&[0x01, 0x03, 0x07, 0xD0, 0x00, 0x0D, 0x84, 0x82]);

        assert_eq!(test, correct);
    }
}
