/// Трейт для запуска преобразования
pub trait DeriveItemProcess<TMsg>: Send + Sync {
    /// Функция преобразования
    fn process(&mut self, msg: &TMsg) -> Option<Vec<TMsg>>;
}
