use super::Row;

/// Внутреннее сообщение между задачами компонента cmp_timescaledb
pub enum InnerMessage {
    /// Строковые данные для сохранения в базе данных
    Rows(Vec<Row>),
    /// Отправить данные из кеша в базу данных по таймеру
    SendByTimer,
}
