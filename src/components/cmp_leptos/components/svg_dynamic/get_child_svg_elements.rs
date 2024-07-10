use leptos::*;
use tracing::info;
use wasm_bindgen::JsCast;

/// Находим вложенные элементы svg корневого элемента
pub fn get_child_svg_elements(root: &web_sys::SvgElement) -> Vec<web_sys::SvgElement> {
    let mut svg_elements = vec![];
    for i in 0..root.child_element_count() {
        info!("element");
        let node = root
            .child_nodes()
            .item(i)
            .unwrap()
            .dyn_into::<web_sys::SvgElement>()
            .unwrap();
        svg_elements.push(node);
    }
    svg_elements
}
