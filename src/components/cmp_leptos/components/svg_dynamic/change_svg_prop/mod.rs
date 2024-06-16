//! Функции для изменения свойств элементов svg-файла

use web_sys::wasm_bindgen::JsCast;

use super::super::material_theme::MaterialTheme;

pub fn text_color(element: &web_sys::SvgElement, new_color: MaterialTheme) -> Option<()> {
    let tspan = element.child_nodes().item(0)?;
    let tspan = tspan.dyn_into::<web_sys::SvgElement>();
    let Ok(tspan) = tspan else { return None };
    fill(&tspan, new_color)?;
    Some(())
}

pub fn text_content(element: &web_sys::SvgElement, new_content: &str) -> Option<()> {
    let tspan = element.child_nodes().item(0)?;
    let tspan = tspan.dyn_into::<web_sys::SvgElement>();
    let Ok(tspan) = tspan else { return None };
    tspan.set_inner_html(new_content);
    Some(())
}

pub fn fill(element: &web_sys::SvgElement, new_fill: MaterialTheme) -> Option<()> {
    element
        .style()
        .set_property("fill", &new_fill.css_var())
        .unwrap();
    Some(())
}

pub fn stroke(element: &web_sys::SvgElement, new_fill: MaterialTheme) -> Option<()> {
    element
        .style()
        .set_property("stroke", &new_fill.css_var())
        .unwrap();
    Some(())
}
