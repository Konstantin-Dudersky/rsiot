use std::{net::IpAddr, time::Duration};

use crate::components_config::master_device::DeviceTrait;

// ANCHOR: Config
/// Конфигурация cmp_modbus_client
#[derive(Debug)]
pub struct Config<TMsg> {
    /// true - включение опроса
    pub enabled: bool,

    /// Вектор настроек коммуникации с устройствами
    ///
    /// Порядок элементов в этом массиве должен соответствовать порядку устройств в массиве devices
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,

    /// Вектор драйверов опрашиваемых устройств
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, super::FieldbusRequest, super::FieldbusResponse>>>,
}
// ANCHOR: Config

// ANCHOR: ConfigDevicesCommSettings
/// Настройки коммуникации с устройствами
#[derive(Clone, Copy, Debug)]
pub struct ConfigDevicesCommSettings {
    /// Настройки подключения к опрашиваемому устройтву
    pub client_type: ClientType,

    /// Unit ID опрашиваемого устройства
    pub unit_id: u8,

    /// Таймаут ожидания ответа
    pub timeout: Duration,

    /// Максимальное количество одновременных соединений с данным устройством
    pub concurrent_connections: u8,
}
// ANCHOR: ConfigDevicesCommSettings

// ANCHOR: ClientType
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
// ANCHOR: ClientType
