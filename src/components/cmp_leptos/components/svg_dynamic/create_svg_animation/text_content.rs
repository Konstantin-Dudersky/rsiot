use leptos::*;
use web_sys::SvgElement;

use super::super::change_svg_prop;

pub fn text_content(svg_element: &SvgElement, text: Signal<String>) {
    let text = text.get();
    change_svg_prop::text_content_2(svg_element, &text);
}
