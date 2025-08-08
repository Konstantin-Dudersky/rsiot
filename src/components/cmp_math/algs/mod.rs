pub mod last_over_time_window;
pub mod simple_moving_average;

use std::time::Duration;

use super::{Error, IntMsgBound, Result};

/// Перечень алгоритмов для обработки данных
pub enum Algs<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    // ANCHOR: LastOverTimeWindow
    /// Выборка последних значений в каждом периоде времени
    LastOverTimeWindow {
        /// Входящие сообщения
        fn_input_value: fn(TIntMsg) -> Option<f64>,
        /// Период времени, за который выбирается последнее значение
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        fn_input_window: fn(TIntMsg) -> Option<Duration>,

        /// Исходящие сообщения
        fn_output: fn(f64) -> TIntMsg,
    },
    // ANCHOR: LastOverTimeWindow
    // ANCHOR: SimpleMovingAverage
    /// Простое скользящее среднее
    SimpleMovingAverage {
        /// Входящие сообщения
        fn_input_value: fn(TIntMsg) -> Option<f64>,
        /// Количество значений
        ///
        /// Константа: `|_| Some(100)`
        fn_input_count: fn(TIntMsg) -> Option<usize>,
        /// Исходящие сообщения
        fn_output: fn(f64) -> TIntMsg,
    },
    // ANCHOR: SimpleMovingAverage
}
