use leptos::prelude::*;
use palette::Srgb;
use web_sys::SvgElement;

use crate::components::cmp_leptos::components::MaterialTheme;

use super::super::change_svg_prop;

pub fn fill(svg_element: &SvgElement, color: Signal<MaterialTheme>) {
    let color = color.get();
    change_svg_prop::fill(svg_element, color).unwrap();
}

pub fn fill_color(svg_element: &SvgElement, color: Signal<Srgb<u8>>) {
    let color = color.get();
    let color = format!("rgb({}, {}, {})", color.red, color.green, color.blue);
    svg_element.style().set_property("fill", &color).unwrap();
}
