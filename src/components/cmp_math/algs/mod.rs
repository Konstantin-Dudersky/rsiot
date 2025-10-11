//! Алгоритмы

#[allow(unused)]
mod _alg_template;
pub(crate) mod derivative;
pub mod downsampling;
pub(crate) mod ema;
pub(crate) mod last_over_time_window;
pub(crate) mod sma;
pub mod statistic;

use crate::message::{MsgDataBound, ValueTime};

pub use {derivative::Gamma, ema::EmaKind};

use tokio::sync::mpsc;

use std::time::Duration;

use super::Error;

type AlgInput = mpsc::Receiver<ValueTime>;
type AlgOutput = mpsc::Sender<ValueTime>;
type AlgFnOutputMsgbus<TMsg, OV> = fn(&OV) -> Option<TMsg>;

/// Перечень алгоритмов для обработки данных
pub enum Algs<TMsg>
where
    TMsg: MsgDataBound,
{
    // ANCHOR: Derivative
    /// Дифференциальное значение
    Derivative {
        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        time_window: Duration,

        /// Время нормализации.
        ///
        /// Например, чтобы определить расход в час, задаём `Duration::from_secs(3600)`
        normalization_time: Duration,

        /// Коэффициент
        gamma: derivative::Gamma,

        /// Функция создания выходного значения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, derivative::OutputValue>,
    },
    // ANCHOR: Derivative

    // ANCHOR: Downsampling
    /// Прореживание
    Downsampling {
        /// Окно времени
        time_window: Duration,

        /// Функция создания выходного значения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, downsampling::OutputValue>,
    },
    // ANCHOR: Downsampling

    // ANCHOR: EMA
    /// Экспоненциальное скользящее среднее
    EMA {
        /// Вид алгоритма
        kind: ema::EmaKind,

        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        time_window: Duration,

        /// Функция создания выходного значения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, ema::OutputValue>,
    },
    // ANCHOR: EMA

    // ANCHOR: LastOverTimeWindow
    /// Выборка последних значений в каждом периоде времени
    LastOverTimeWindow {
        /// Период времени, за который выбирается последнее значение
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        time_window: Duration,

        /// Исходящие сообщения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, f64>,
    },
    // ANCHOR: LastOverTimeWindow

    // ANCHOR: SMA
    /// Простое скользящее среднее
    ///
    /// TODO: Текущая реализация - last. Проработать next и linear.
    SMA {
        /// Функция нахождения окна времени
        ///
        /// Константа: `|_| Some(Duration::from_millis(100))`
        time_window: Duration,

        /// Функция создания выходного значения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, sma::OutputValue>,
    },
    // ANCHOR: SMA

    // ANCHOR: Statistic
    /// Статистиска
    Statistic {
        /// Окно времени
        time_window: Duration,

        /// Выбор индикаторов
        indicators: statistic::Indicators,

        /// Функция создания выходного значения
        fn_output_msgbus: AlgFnOutputMsgbus<TMsg, statistic::OutputValue>,
    },
    // ANCHOR: Statistic
}
