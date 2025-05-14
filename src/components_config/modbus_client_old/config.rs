use std::net::IpAddr;

use super::{InputConfig, PeriodicConfig};

/// Конфигурация cmp_modbus_client
#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    /// true - разрешение работы
    pub enabled: bool,

    /// Настройки подключения к опрашиваемому устройтву
    pub connection_config: ClientType,

    /// Адрес подчиненного устройства (обычно 1)
    pub unit_id: u8,

    /// Конфигурация запросов на основе входных сообщений
    pub input_config: Vec<InputConfig<TMessage>>,

    /// Конфигурация периодических запросов
    pub periodic_config: Vec<PeriodicConfig<TMessage>>,
}

/// Конфигурация Modbus клиента
#[derive(Clone, Debug)]
pub enum ClientType {
    /// Вариант для Modbus TCP
    Tcp(TcpClientType),
    /// Вариант для Modbus RTU
    Rtu,
}

/// Конфигурация Modbus TCP клиента
#[derive(Clone, Debug)]
pub struct TcpClientType {
    /// IP-адрес устройства
    pub host: IpAddr,

    /// Порт устройства (обычно 502)
    pub port: u16,
}
