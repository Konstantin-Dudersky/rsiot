//! Общие данные для компонентов cmp_linux_uart_master и cmp_linux_uart_slave

use linux_embedded_hal::serialport;

use crate::components_config::uart_general::{
    Baudrate, DataBits, FieldbusRequest, FieldbusResponse, Parity, StopBits,
};

impl From<DataBits> for serialport::DataBits {
    fn from(value: DataBits) -> Self {
        match value {
            DataBits::_5 => serialport::DataBits::Five,
            DataBits::_6 => serialport::DataBits::Six,
            DataBits::_7 => serialport::DataBits::Seven,
            DataBits::_8 => serialport::DataBits::Eight,
        }
    }
}

impl From<Parity> for serialport::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => serialport::Parity::None,
            Parity::Even => serialport::Parity::Even,
            Parity::Odd => serialport::Parity::Odd,
        }
    }
}

impl From<StopBits> for serialport::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::_1 => serialport::StopBits::One,
            StopBits::_1p5 => unimplemented!("Stop bit 1.5 not implemented"),
            StopBits::_2 => serialport::StopBits::Two,
        }
    }
}
