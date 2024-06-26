use leptos::*;
use palette::Srgb;

use crate::components::cmp_plc::plc::library;

#[derive(Clone)]
pub(crate) enum SvgInputSignal {
    Fill(Signal<Srgb<u8>>),
    Y(Signal<f64>),
    TextContent(Signal<String>),
    PlcDrivesMotor(Signal<library::drives::motor::QHmiStatus>),
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
    pub fn fill(id: &str, signal: impl Into<Signal<Srgb<u8>>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::Fill(signal.into()),
        }
    }

    /// Смещение по оси Y
    pub fn y(id: &str, signal: impl Into<Signal<f64>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::Y(signal.into()),
        }
    }

    /// Текстовое содержение
    pub fn text_content(id: &str, signal: impl Into<Signal<String>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::TextContent(signal.into()),
        }
    }

    pub fn plc_drives_motor(
        id: &str,
        signal: impl Into<Signal<library::drives::motor::QHmiStatus>>,
    ) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::PlcDrivesMotor(signal.into()),
        }
    }
}
