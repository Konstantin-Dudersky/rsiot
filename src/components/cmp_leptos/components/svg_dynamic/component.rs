use leptos::*;
use tracing::warn;
use wasm_bindgen::{JsCast, JsValue};

use super::{svg_input::SvgInputSignal, SvgInput};

#[component]
pub fn SvgDynamic(file: &'static str, svg_input: Vec<SvgInput>) -> impl IntoView {
    let div_ref = create_node_ref();
    div_ref.on_load(move |_| {
        for input in svg_input {
            create_effect(move |_| {
                let element = document().get_element_by_id(&input.id);
                let element = match element {
                    Some(val) => val,
                    None => {
                        warn!("Element with id {} not found", input.id);
                        return;
                    }
                };

                let svg_element = element.dyn_into::<web_sys::SvgElement>();
                let svg_element = match svg_element {
                    Ok(val) => val,
                    Err(_) => {
                        warn!("Cannot cast element with id {} into SvgElement", input.id);
                        return;
                    }
                };

                let result = change_svg_element(&input, &svg_element);
                if let Err(err) = result {
                    warn!("Cannot set attribute TODO of svg element with id {err:?}")
                };
            });
        }
    });

    view! { <div node_ref=div_ref inner_html=file></div> }
}

fn change_svg_element(svg_input: &SvgInput, element: &web_sys::SvgElement) -> Result<(), JsValue> {
    match svg_input.signal {
        SvgInputSignal::Fill(sig) => {
            let value = sig.get();
            let value = format!("#{:x}", value);
            element.style().set_property("fill", &value)
        }

        SvgInputSignal::TextContent(sig) => {
            element.set_inner_html(&sig.get());
            Ok(())
        }

        SvgInputSignal::Y(sig) => {
            let value = sig.get().to_string();
            element.set_attribute("y", &value)
        }
    }
}
