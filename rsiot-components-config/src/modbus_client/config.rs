use std::net::IpAddr;

use super::{InputConfig, PeriodicConfig};

/// Конфигурация Modbus клиента
#[derive(Clone, Debug)]
pub enum Config<TMessage> {
    /// Вариант для Modbus TCP
    Tcp(TcpClientConfig<TMessage>),
    /// Вариант для Modbus RTU
    Rtu,
}

/// Конфигурация Modbus TCP клиента
#[derive(Clone, Debug)]
pub struct TcpClientConfig<TMessage> {
    /// IP-адрес устройства
    pub host: IpAddr,
    /// Порт устройства (обычно 502)
    pub port: u16,
    /// Адрес подчиненного устройства (обычно 1)
    pub unit_id: u8,
    /// Конфигурация запросов на основе входных сообщений
    pub input_config: Vec<InputConfig<TMessage>>,
    /// Конфигурация периодических запросов
    pub periodic_config: Vec<PeriodicConfig<TMessage>>,
}
