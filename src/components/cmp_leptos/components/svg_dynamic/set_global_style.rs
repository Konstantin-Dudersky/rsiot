//! Изменить стили всех элементов файла svg. Полезно для переключения темы

use leptos::{leptos_dom::helpers::document, wasm_bindgen::JsCast};

use super::{super::material_theme::MaterialTheme, change_svg_prop, Error, INK_LABEL};

pub fn set_global_style(svg_id: &str) -> Result<(), Error> {
    let root_node = document()
        .get_element_by_id(svg_id)
        .ok_or_else(|| Error::ElementNotFound(svg_id.to_string()))?
        .parent_node()
        .unwrap();

    node_process(root_node);
    Ok(())
}

/// Рекурсивно проходим по всем узлам документа
fn node_process(node: web_sys::Node) {
    let element = node.dyn_ref::<web_sys::SvgElement>();
    if let Some(element) = element {
        element_process(element)
    };
    let node_list = node.child_nodes();
    for i in 0..node_list.length() {
        let node = node_list.item(i).unwrap();
        node_process(node)
    }
}

fn element_process(element: &web_sys::SvgElement) {
    let label = element.get_attribute(INK_LABEL);
    let Some(label) = label else { return };
    match label.as_str() {
        "text" => {
            change_svg_prop::text_color(element, MaterialTheme::sys_color_on_surface);
        }
        "stroke" => {
            change_svg_prop::stroke(element, MaterialTheme::sys_color_on_surface);
        }
        "fill" => {
            change_svg_prop::fill(element, MaterialTheme::sys_color_on_surface);
        }
        _ => (),
    }
}
