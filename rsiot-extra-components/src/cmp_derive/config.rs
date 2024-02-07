use rsiot_messages_core::IMessage;

use super::derive_item_process::DeriveItemProcess;

pub struct Config<TMsg>
where
    TMsg: IMessage,
{
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
