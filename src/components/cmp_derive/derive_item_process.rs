use crate::message::Message;

/// Трейт для запуска преобразования
pub trait DeriveItemProcess<TMsg>: Send + Sync {
    /// Функция преобразования
    fn process(&mut self, msg: &Message<TMsg>) -> Option<Vec<Message<TMsg>>>;
}
