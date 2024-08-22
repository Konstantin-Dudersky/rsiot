//! Пример реализации перечисления сервисов

use super::ServiceBound;

/// Пример перечисления сервисов
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Service {
    /// Пример сервиса
    example_service,
}

impl ServiceBound for Service {}
