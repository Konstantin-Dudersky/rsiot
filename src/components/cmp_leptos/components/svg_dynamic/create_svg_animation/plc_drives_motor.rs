use leptos::*;
use web_sys::SvgElement;

use crate::components::cmp_plc::plc::library::drives::motor::{QHmiStatus, QState};

use super::{
    super::{super::material_theme::MaterialTheme, change_svg_prop, get_child_svg_elements},
    plc_drives_shared,
};

pub fn plc_drives_motor(svg_element: &SvgElement, hmi_status: Signal<QHmiStatus>) {
    let svg_elements = get_child_svg_elements(svg_element);

    for element in svg_elements {
        let label = element.get_attribute("inkscape:label");
        let Some(label) = label else { continue };
        match label.as_str() {
            "mode" => plc_drives_shared::mode(&element, hmi_status.get().mode),
            "mode_text" => plc_drives_shared::mode_text(&element, hmi_status.get().mode),
            "state" => match hmi_status.get().state {
                QState::Stop => {
                    change_svg_prop::fill(&element, MaterialTheme::sys_color_surface);
                    change_svg_prop::stroke(&element, MaterialTheme::sys_color_on_surface)
                }
                .unwrap(),
                QState::Start => {
                    change_svg_prop::fill(&element, MaterialTheme::extended_color_green_color);
                    change_svg_prop::stroke(&element, MaterialTheme::extended_color_green_on_color)
                }
                .unwrap(),
                QState::Alarm => {
                    todo!()
                }
            },

            "state-text" => match hmi_status.get().state {
                QState::Stop => {
                    change_svg_prop::text_color(&element, MaterialTheme::sys_color_on_surface)
                        .unwrap()
                }
                QState::Start => change_svg_prop::text_color(
                    &element,
                    MaterialTheme::extended_color_green_on_color,
                )
                .unwrap(),
                QState::Alarm => todo!(),
            },

            _ => continue,
        }
    }
}
