/// Ошибки работы с сообщениями
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Message deserialization error
    #[error("Message deserialization\nError: {error}\nData: {data}")]
    Deserialization {
        /// Текст ошибки
        error: String,
        /// Данные, которые не удалось десериализовать
        data: String,
    },

    /// Message serialization error
    #[error("Message serialization error: {0}")]
    Serialization(String),
}
