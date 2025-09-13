pub(crate) mod derivative;
// pub(crate) mod differential_nw;
pub(crate) mod ema;
pub(crate) mod last_over_time_window;
pub(crate) mod simple_moving_average;
pub(crate) mod sma;

pub use {derivative::Gamma, ema::EmaKind};

use tokio::sync::{broadcast, mpsc};

use std::time::Duration;

use time::OffsetDateTime;

use super::{Error, IntMsgBound, Result};

type AlgInput<TIntMsg> = broadcast::Receiver<TIntMsg>;
type AlgOutput<TIntMsg> = mpsc::Sender<TIntMsg>;

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
    // ANCHOR: SMA
    /// Простое скользящее среднее
    ///
    /// TODO: Текущая реализация - last. Проработать next и linear.
    SMA {
        /// Функция извлечения значения
        fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,

        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        fn_input_time_window: fn(TIntMsg) -> Option<Duration>,

        /// Функция создания выходного значения
        fn_output: fn(sma::OutputValue) -> TIntMsg,
    },
    // ANCHOR: SMA
    // ANCHOR: EMA
    /// Экспоненциальное скользящее среднее
    EMA {
        /// Вид алгоритма
        kind: ema::EmaKind,

        /// Функция извлечения значения
        fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,

        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        fn_input_time_window: fn(TIntMsg) -> Option<Duration>,

        /// Функция создания выходного значения
        fn_output: fn(ema::OutputValue) -> TIntMsg,
    },
    // ANCHOR: EMA
    // ANCHOR: Derivative
    /// Дифференциальное значение
    Derivative {
        /// Функция извлечения значения
        fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,

        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        fn_input_time_window: fn(TIntMsg) -> Option<Duration>,

        /// Время нормализации.
        ///
        /// Например, чтобы определить расход в час, задаём `Duration::from_secs(3600)`
        normalization_time: Duration,

        /// Коэффициент
        gamma: derivative::Gamma,

        /// Функция создания выходного значения
        fn_output: fn(derivative::OutputValue) -> TIntMsg,
    },
    // ANCHOR: Derivative
}
