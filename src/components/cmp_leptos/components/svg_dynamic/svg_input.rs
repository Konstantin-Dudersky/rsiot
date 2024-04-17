use leptos::*;
use palette::Srgb;

pub(crate) enum SvgInputSignal {
    Fill(Signal<Srgb<u8>>),
    Y(Signal<f64>),
    TextContent(Signal<String>),
}

/// Изменение свойств элементов SVG
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
}
