use std::{rc::Rc, sync::Mutex};

use ev::Event;
use gloo::dialogs::alert;
use leptos::*;
use tracing::{info, warn};
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use super::create_svg_animation;

use super::{
    set_global_style::set_global_style,
    {svg_input::SvgInputSignal, SvgInput, SvgOutput},
};

#[component]
pub fn SvgDynamic<FOutput>(
    file: &'static str,
    svg_input: Vec<SvgInput>,
    svg_output: SvgOutput<FOutput>,
) -> impl IntoView
where
    FOutput: Fn(&str) + 'static,
{
    let id = format!("svg_{}", Uuid::new_v4());
    let div_ref = create_node_ref();

    let output_callback = Rc::new(Mutex::new(Closure::wrap(Box::new(move |e: Event| {
        let id = extract_id_from_event(e).unwrap();
        (svg_output.callback)(&id)
    }) as Box<dyn FnMut(_)>)));

    let id_clone = id.clone();

    div_ref.on_load(move |_| {
        // Задаем стили элементов svg файла
        create_effect(move |_| {
            let id_clone = id_clone.clone();
            set_global_style(id_clone)
        });

        for input in svg_input {
            create_effect(move |_| create_effect_for_svg_input(&input).unwrap());
        }

        for id in svg_output.ids {
            let cb_clone = output_callback.clone();
            // Не знаю, зачем нужно оборачивать в create_effect, но без него `get_element_by_id` не
            // находится
            create_effect(move |_| {
                let Some(svg_element) = get_svg_element_by_id(&id) else {
                    return;
                };

                info!("Регистрация коллбеков для вызова");

                let lock = cb_clone.lock().unwrap();

                svg_element
                    .add_event_listener_with_callback("click", lock.as_ref().unchecked_ref())
                    .unwrap();
                svg_element
                    .style()
                    .set_property("cursor", "pointer")
                    .unwrap();
            });
        }
    });

    view! { <div id=id node_ref=div_ref inner_html=file></div> }
}

#[deprecated]
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

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesMotor(_) => Ok(()),

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesValveAnalog(_) => Ok(()),

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesValve(_) => Ok(()),
    }
}

/// Извлечение id элемента из события
fn extract_id_from_event(event: Event) -> Option<String> {
    let target = event.current_target()?;

    let element = target.dyn_into::<web_sys::Element>();
    let element = match element {
        Ok(val) => val,
        Err(err) => {
            let err = format!("{:?}", err);
            let err = format!("Cannot cast element: {:?}", err);
            warn_and_alert(err);
            return None;
        }
    };

    let id = element.id();
    Some(id)
}

/// Находим элемент svg по id
fn get_svg_element_by_id(id: &str) -> Option<web_sys::SvgElement> {
    let element = document().get_element_by_id(id);
    let element = match element {
        Some(val) => val,
        None => {
            let err = format!("Element with id '{}' not found", id);
            warn_and_alert(err);
            return None;
        }
    };

    let svg_element = element.dyn_into::<web_sys::SvgElement>();
    let svg_element = match svg_element {
        Ok(val) => val,
        Err(_) => {
            let err = format!("Cannot cast element with id '{} 'into SvgElement", id);
            warn_and_alert(err);
            return None;
        }
    };

    Some(svg_element)
}

/// Вывести сообщение в консоль и в окно браузера (alert)
fn warn_and_alert(text: impl AsRef<str>) {
    warn!("{}", text.as_ref());
    alert(text.as_ref());
}

fn create_effect_for_svg_input(input: &SvgInput) -> Option<()> {
    let svg_element = get_svg_element_by_id(&input.id)?;

    match input.signal {
        SvgInputSignal::Fill(_) => todo!(),

        SvgInputSignal::Y(_) => todo!(),

        SvgInputSignal::TextContent(text) => create_svg_animation::text_content(&svg_element, text),

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesMotor(hmi_status) => {
            create_svg_animation::plc_drives_motor(&svg_element, hmi_status)
        }

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesValveAnalog(hmi_status) => {
            create_svg_animation::plc_drives_valve_analog(&svg_element, hmi_status)?
        }

        #[cfg(feature = "cmp_plc")]
        SvgInputSignal::PlcDrivesValve(hmi_status) => {
            create_svg_animation::plc_drives_valve(&svg_element, hmi_status)?
        }
    }

    let result = change_svg_element(input, &svg_element);
    if let Err(err) = result {
        warn!(
            "Cannot set attribute TODO of svg element with id '{}' :{:?}",
            input.id, err
        )
    };
    Some(())
}
