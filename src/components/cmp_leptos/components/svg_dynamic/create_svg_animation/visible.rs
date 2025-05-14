use leptos::prelude::*;
use web_sys::SvgElement;

pub fn visible(svg_element: &SvgElement, visible: Signal<bool>) {
    let visible = visible.get();

    let visibility = match visible {
        true => "visible",
        false => "hidden",
    };

    svg_element
        .style()
        .set_property("visibility", visibility)
        .unwrap();
}
