use std::time::Instant;

use crate::components_config::master_device::RequestResponseBound;

/// Структура отдельного запроса на коммуникацию по шине Modbus
#[derive(Clone, Debug)]
pub struct FieldbusRequest {
    /// Время создания запроса.
    ///
    /// Можно контролировать время выполнения запросов
    pub request_creation_time: Instant,

    /// Вид запроса.
    ///
    /// Необходим для правильной расшифровки ответа
    pub request_kind: u8,

    /// Массив операций
    pub operation: RequestContent,
}

impl FieldbusRequest {
    /// Создание запроса. Адрес задается позже
    pub fn new(request_kind: impl Into<u8>, operation: RequestContent) -> Self {
        Self {
            request_creation_time: Instant::now(),
            request_kind: request_kind.into(),
            operation,
        }
    }
}

impl RequestResponseBound for FieldbusRequest {}

/// Параметры запроса Modbus
#[derive(Clone, Debug)]
pub enum RequestContent {
    /// Чтение регистров флагов
    ReadCoils {
        /// Начальный адрес
        start_address: u16,
        /// Количество
        count: u16,
    },
    /// Чтение регистров хранения
    ReadHoldingRegisters {
        /// Начальный адрес
        start_address: u16,
        /// Количество
        count: u16,
    },
    /// Чтение регистров ввода
    ReadInputRegisters {
        /// Начальный адрес
        start_address: u16,
        /// Количество
        count: u16,
    },
    /// Запись одного регистра хранения
    WriteSingleRegister {
        /// Адрес
        address: u16,
        /// Значение
        value: u16,
    },
}
