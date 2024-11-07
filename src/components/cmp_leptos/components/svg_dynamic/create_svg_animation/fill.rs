use leptos::prelude::*;
use web_sys::SvgElement;

use crate::components::cmp_leptos::components::MaterialTheme;

use super::super::change_svg_prop;

pub fn fill(svg_element: &SvgElement, color: Signal<MaterialTheme>) {
    let color = color.get();
    change_svg_prop::fill(svg_element, color).unwrap();
}
