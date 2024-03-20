/// Ошибки cmp_system_info
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Невозможно определить данные
    #[error("Cannot define: {field}")]
    CannotDefine {
        /// Название поля
        field: String,
    },
}
