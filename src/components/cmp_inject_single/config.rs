use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_inject_single
pub struct Config<TMsg, TFnSingle>
where
    TMsg: MsgDataBound,
    TFnSingle: FnOnce() -> Vec<TMsg> + Send + Sync,
{
    /// Создание исходящих сообщений
    pub fn_output: TFnSingle,
}
