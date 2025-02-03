use std::fmt::Debug;

use super::AddressBound;

/// Ограничения для структур запросов и ответов
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug)]
/// ```
pub trait RequestResponseBound<TAddress>
where
    Self: Clone + Debug + Send + Sync,
    TAddress: AddressBound,
{
    /// Адрес устройства
    fn address(&self) -> TAddress;

    /// Задать адрес устройства
    fn set_address(&mut self, address: TAddress);
}
