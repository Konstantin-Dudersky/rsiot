use std::fmt::Debug;

/// Ограничения для перечисления сервисов
///
/// Добавить макросы для стуркуры:
/// ```rust
/// #[derive(Debug, Clone, PartialEq)]
/// ```
pub trait ServiceBound: Debug + Clone + PartialEq + Send {
    /// Возвращает строку для добавления в трассировку
    fn trace_name(&self) -> String {
        let full_str = format!("{:?}", self);
        full_str
    }
}

/// Запуск:
///
/// ```bash
/// cargo test --target="x86_64-unknown-linux-gnu" -- message::service::tests
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_name() {
        #[derive(Debug, Clone, PartialEq)]
        enum Services {
            Service1,
        }

        impl ServiceBound for Services {}

        assert_eq!("Service1", Services::Service1.trace_name())
    }
}
