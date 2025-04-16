use std::{rc::Rc, sync::Mutex};

use gloo::dialogs::alert;
use leptos::prelude::*;
use leptos::tachys::html::event::Event;
use tracing::warn;
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast};

use super::{create_svg_animation, Error};

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
    FOutput: Fn(&str) + 'static + Clone,
{
    let id = format!("svg_{}", Uuid::new_v4());
    let div_ref = NodeRef::new();

    let id_clone = id.clone();

    // track_counter отслеживает кол-во обновлений элемента div_ref. При полной перезагрузке
    // страницы проход по SVG элементам выполняется сразу без проблем. При переходе между
    // страницами, необходимо подождать, пока track_counter будет равен 2. Почему 2? ХЗ
    let mut track_counter = 0;
    let mut inited = false;
    Effect::new(move || {
        div_ref.track();
        track_counter += 1;
        if !inited {
            let id_clone = id_clone.clone();
            let svg_input = svg_input.clone();
            let svg_output = svg_output.clone();
            let res = setup_svg_input_and_output(id_clone, svg_input, svg_output);
            match res {
                Ok(_) => inited = true,
                Err(err) if track_counter >= 2 => warn!("Error: {}", err),
                _ => (),
            }
        }
    });

    view! {
        <div id=id node_ref=div_ref inner_html=file></div>
    }
}

fn setup_svg_input_and_output<FOutput>(
    id_clone: String,
    svg_input: Vec<SvgInput>,
    svg_output: SvgOutput<FOutput>,
) -> Result<(), Error>
where
    FOutput: Fn(&str) + 'static,
{
    // Задаем стили элементов svg файла
    set_global_style(&id_clone)?;

    // Создаем эффекты для анимации svg
    for input in svg_input {
        let input_clone = input.clone();
        Effect::new(move |_| create_effect_for_svg_input(&input_clone).unwrap());
    }

    let output_callback = Rc::new(Mutex::new(Closure::wrap(Box::new(move |e: Event| {
        let id = extract_id_from_event(e).unwrap();
        (svg_output.callback)(&id)
    }) as Box<dyn FnMut(_)>)));

    // Создаем обработчики событий из svg
    for id in &svg_output.ids {
        let id_clone = id.clone();

        let cb_clone = output_callback.clone();
        // Не знаю, зачем нужно оборачивать в create_effect, но без него `get_element_by_id` не
        // находится
        Effect::new(move |_| {
            let Some(svg_element) = get_svg_element_by_id(&id_clone) else {
                return;
            };

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

    Ok(())
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
        SvgInputSignal::Fill(color) => create_svg_animation::fill(&svg_element, color),

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
    Some(())
}
