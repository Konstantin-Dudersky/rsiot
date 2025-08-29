use std::time::Duration;

// ANCHOR: Config
/// Конфигурация cmp_inject_periodic
#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnPeriodic>
where
    TMsg: Clone,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    /// Период вызова
    pub period: Duration,

    /// Функция для генерирования сообщений
    ///
    /// Тип данных - `FnMut() -> Vec<TMsg>`
    pub fn_periodic: TFnPeriodic,
}
// ANCHOR: Config
