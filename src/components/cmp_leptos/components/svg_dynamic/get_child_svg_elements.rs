use leptos::*;
use tracing::warn;
use wasm_bindgen::JsCast;

/// Находим вложенные элементы svg корневого элемента
pub fn get_child_svg_elements(root: &web_sys::SvgElement) -> Vec<web_sys::SvgElement> {
    let mut svg_elements = vec![];
    for i in 0..root.child_element_count() {
        let node = root.child_nodes().item(i).unwrap();
        let node_name = node.node_name();

        let svg_element = node.dyn_into::<web_sys::SvgElement>();
        let svg_element = match svg_element {
            Ok(svg_element) => svg_element,
            Err(_) => {
                let err = format!(
                    "Error converting node into web_sys::SvgElemet; node name: {}",
                    node_name
                );
                warn!("{}", err);
                continue;
            }
        };

        svg_elements.push(svg_element);
    }
    svg_elements
}
