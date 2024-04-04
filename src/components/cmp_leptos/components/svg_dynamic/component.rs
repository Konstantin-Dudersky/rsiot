use leptos::*;
use tracing::warn;
use wasm_bindgen::{JsCast, JsValue};

use super::{SvgInput, SvgInputSignal};

#[component]
pub fn SvgDynamic(file: &'static str, svg_input: Vec<SvgInput>) -> impl IntoView {
    let div_ref = create_node_ref();
    div_ref.on_load(move |_| {
        for input in svg_input {
            create_effect(move |_| {
                let element = document().get_element_by_id(input.get_id());
                let element = match element {
                    Some(val) => val,
                    None => {
                        warn!("Element with id {} not found", input.get_id());
                        return;
                    }
                };

                let svg_element = element.dyn_into::<web_sys::SvgElement>();
                let svg_element = match svg_element {
                    Ok(val) => val,
                    Err(_) => {
                        warn!(
                            "Cannot cast element with id {} into SvgElement",
                            input.get_id()
                        );
                        return;
                    }
                };

                let result = change_svg_element_prop(&input, &svg_element);
                if let Err(err) = result {
                    warn!("Cannot set attribute TODO of svg element with id {err:?}")
                };
            });
        }
    });

    view! { <div node_ref=div_ref inner_html=file></div> }
}

fn change_svg_element_prop(
    svg_input: &SvgInput,
    element: &web_sys::SvgElement,
) -> Result<(), JsValue> {
    match svg_input.get_signal() {
        SvgInputSignal::EllipseFill(sig) => {
            let value = sig.get();
            let value = format!("#{:x}", value);
            element.style().set_property("fill", &value)
        }

        SvgInputSignal::RectY(sig) => {
            let value = sig.get().to_string();
            element.set_attribute("y", &value)
        }

        SvgInputSignal::TextContent(sig) => {
            element.set_inner_html(&sig.get());
            Ok(())
        }
    }
}
