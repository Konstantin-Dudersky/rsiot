use std::time::Duration;

use super::BufferBound;

pub type FnInput<TMsg, TBuffer> = fn(&TMsg, &TBuffer) -> Option<TBuffer>;
pub type FnOutput<TMsg, TBuffer> = fn(&TBuffer) -> TMsg;

/// Настройки cmp_derive
pub struct Config<TMsg, TBuffer>
where
    TBuffer: BufferBound,
{
    /// Когда создавать исходящие сообщения
    pub output_send: ConfigOutputSend,

    /// Обработка входящих сообщений и сохранение в буфере
    pub fn_input: FnInput<TMsg, TBuffer>,

    /// Формирование исходящих сообщений на основе данных, сохраненных в `store`
    pub fn_output: fn(&TBuffer) -> TMsg,
}

/// Настройка создания исходящих сообщений
pub enum ConfigOutputSend {
    /// Создавать по каждому изменению буфера
    OnEveryChange,

    /// Создавать периодически с заданным интервалом
    Periodic(Duration),
}
