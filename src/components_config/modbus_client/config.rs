use std::{net::IpAddr, time::Duration};

use crate::components_config::master_device::DeviceTrait;

/// Конфигурация cmp_modbus_client
#[derive(Debug)]
pub struct Config<TMsg> {
    /// true - разрешение работы
    pub enabled: bool,

    /// Массив настроек коммуникации с устройствами
    ///
    /// Порядок элементов в этом массиве должен соответствовать порядку устройств в массиве devices
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,

    /// Драйвера устройств
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, super::FieldbusRequest, super::FieldbusResponse>>>,
}

/// Настройки коммуникации с устройствами
#[derive(Clone, Copy, Debug)]
pub struct ConfigDevicesCommSettings {
    /// Настройки подключения к опрашиваемому устройтву
    pub client_type: ClientType,

    /// Адрес подчиненного устройства (обычно 1)
    pub unit_id: u8,

    /// Таймаут
    pub timeout: Duration,

    /// Допустимое кол-во одновременных соединений
    pub concurrent_connections: u8,
}

/// Конфигурация Modbus клиента
#[derive(Clone, Copy, Debug)]
pub enum ClientType {
    /// Вариант для Modbus TCP
    Tcp {
        /// IP-адрес устройства
        host: IpAddr,

        /// Порт устройства (обычно 502)
        port: u16,
    },
    /// Вариант для Modbus RTU
    Rtu,
}
