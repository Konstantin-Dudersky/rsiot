use leptos::*;

use crate::components::cmp_leptos::components::MaterialTheme;
#[cfg(feature = "cmp_plc")]
use crate::components::cmp_plc::plc::library;

#[derive(Clone)]
pub(crate) enum SvgInputSignal {
    Fill(Signal<MaterialTheme>),
    TextContent(Signal<String>),

    #[cfg(feature = "cmp_plc")]
    PlcDrivesMotor(Signal<library::drives::motor::QHmiStatus>),

    #[cfg(feature = "cmp_plc")]
    PlcDrivesValveAnalog(Signal<library::drives::valve_analog::QHmiStatus>),

    #[cfg(feature = "cmp_plc")]
    PlcDrivesValve(Signal<library::drives::valve::QHmiStatus>),
}

/// Изменение свойств элементов SVG
#[derive(Clone)]
pub struct SvgInput {
    /// HTML аттрибут id элемента
    pub(crate) id: String,
    /// Сигнал с новым значением свойства
    pub(crate) signal: SvgInputSignal,
}

impl SvgInput {
    /// Заливка цветом
    // pub fn fill(id: &str, signal: impl Into<Signal<Srgb<u8>>>) -> Self {
    pub fn fill(id: &str, signal: impl Into<Signal<MaterialTheme>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::Fill(signal.into()),
        }
    }

    /// Текстовое содержение
    pub fn text_content(id: &str, signal: impl Into<Signal<String>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::TextContent(signal.into()),
        }
    }

    /// Двигатель `Motor`
    #[cfg(feature = "cmp_plc")]
    pub fn plc_drives_motor(
        id: &str,
        signal: impl Into<Signal<library::drives::motor::QHmiStatus>>,
    ) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::PlcDrivesMotor(signal.into()),
        }
    }

    /// Задвижка `ValveAnalog`
    #[cfg(feature = "cmp_plc")]
    pub fn plc_drives_valve_analog(
        id: &str,
        signal: impl Into<Signal<library::drives::valve_analog::QHmiStatus>>,
    ) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::PlcDrivesValveAnalog(signal.into()),
        }
    }

    /// Задвижка `Valve`
    #[cfg(feature = "cmp_plc")]
    pub fn plc_drives_valve(
        id: &str,
        signal: impl Into<Signal<library::drives::valve::QHmiStatus>>,
    ) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::PlcDrivesValve(signal.into()),
        }
    }
}
