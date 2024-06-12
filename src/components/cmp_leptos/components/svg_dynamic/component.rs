use std::sync::{Arc, Mutex};

use ev::Event;
use leptos::*;
use tracing::{info, warn};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::{svg_input::SvgInputSignal, SvgInput, SvgOutput};

#[component]
pub fn SvgDynamic<FOutput>(
    file: &'static str,
    svg_input: Vec<SvgInput>,
    svg_output: SvgOutput<FOutput>,
) -> impl IntoView
where
    FOutput: Fn(&str) -> () + 'static,
{
    let div_ref = create_node_ref();

    let output_callback = Arc::new(Mutex::new(Closure::wrap(Box::new(move |e: Event| {
        let s = e.to_string().as_string().unwrap();
        let e1 = e.target().unwrap();

        info!("{:?}", e.target().unwrap());
        (svg_output.callback)(&s)
    }) as Box<dyn FnMut(_)>)));

    div_ref.on_load(move |_| {
        for input in svg_input {
            let cb_clone = output_callback.clone();

            create_effect(move |_| {
                let element = document().get_element_by_id(&input.id);
                let element = match element {
                    Some(val) => val,
                    None => {
                        warn!("Element with id '{}' not found", input.id);
                        return;
                    }
                };

                let svg_element = element.dyn_into::<web_sys::SvgElement>();
                let svg_element = match svg_element {
                    Ok(val) => val,
                    Err(_) => {
                        warn!("Cannot cast element with id '{} 'into SvgElement", input.id);
                        return;
                    }
                };

                let result = change_svg_element(&input, &svg_element);
                if let Err(err) = result {
                    warn!(
                        "Cannot set attribute TODO of svg element with id '{}' :{:?}",
                        input.id, err
                    )
                };

                let lock = cb_clone.lock().unwrap();
                svg_element
                    .add_event_listener_with_callback("click", &lock.as_ref().unchecked_ref())
                    .unwrap();
            });
        }

        for id in svg_output.ids {
            let cb_clone = output_callback.clone();
            // Не знаю, зачем нужно оборачивать в create_effect, но без него `get_element_by_id` не
            // находится
            create_effect(move |_| {
                let element = document().get_element_by_id(&id);
                let element = match element {
                    Some(val) => val,
                    None => {
                        warn!("Element with id '{}' not found", id);
                        return;
                    }
                };

                let svg_element = element.dyn_into::<web_sys::SvgElement>();
                let svg_element = match svg_element {
                    Ok(val) => val,
                    Err(_) => {
                        warn!("Cannot cast element with id '{}' into SvgElement", id);
                        return;
                    }
                };

                let lock = cb_clone.lock().unwrap();

                svg_element
                    .add_event_listener_with_callback("click", &lock.as_ref().unchecked_ref())
                    .unwrap();
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
