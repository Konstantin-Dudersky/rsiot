use std::fmt::Debug;

/// Ограничения для структур запросов и ответов
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug)]
/// ```
pub trait RequestResponseBound
where
    // Self: Clone + Debug + DeserializeOwned + Send + Serialize + Sync,
    Self: Clone + Debug + Send + Sync,
{
    /// Адрес устройства
    fn address(&self) -> u8;

    /// Задать адрес устройства
    fn set_address(&mut self, address: u8);
}
