use crate::message::MsgDataBound;

/// Настройка компонента cmp_shutdown
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| false
    /// ```
    pub fn_input: fn(TMsg) -> bool,
}
