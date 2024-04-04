use leptos::*;
use palette::Srgb;

pub enum SvgInputSignal {
    EllipseFill(Signal<Srgb<u8>>),
    RectY(Signal<f64>),
    TextContent(Signal<String>),
}

pub struct SvgInput {
    id: String,
    signal: SvgInputSignal,
}

impl SvgInput {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_signal(&self) -> &SvgInputSignal {
        &self.signal
    }

    pub fn ellipse_fill(id: &str, signal: impl Into<Signal<Srgb<u8>>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::EllipseFill(signal.into()),
        }
    }

    pub fn rect_y(id: &str, signal: impl Into<Signal<f64>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::RectY(signal.into()),
        }
    }

    pub fn text_content(id: &str, signal: impl Into<Signal<String>>) -> Self {
        Self {
            id: id.to_string(),
            signal: SvgInputSignal::TextContent(signal.into()),
        }
    }
}
