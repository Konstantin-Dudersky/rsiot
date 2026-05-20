use super::derive_item_process::DeriveItemProcess;

/// Настройки cmp_derive
pub struct Config<TMsg> {
    /// Вектор структур `DeriveItem`
    ///
    /// # Примеры
    ///
    /// ```rust
    /// #[derive(Clone, Default, PartialEq)]
    /// struct NewMsg {
    ///     pub f64: Option<f64>,
    /// }
    /// ```
    pub derive_items: Vec<Box<dyn DeriveItemProcess<TMsg>>>,
}
