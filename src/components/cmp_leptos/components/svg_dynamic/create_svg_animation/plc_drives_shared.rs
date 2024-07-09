use web_sys::SvgElement;

use crate::components::cmp_plc::plc::library::drives::select_mode;

use super::super::{super::material_theme::MaterialTheme, change_svg_prop};

pub fn mode(element: &SvgElement, mode: select_mode::QMode) {
    match mode {
        select_mode::QMode::Auto => {
            change_svg_prop::fill(element, MaterialTheme::extended_color_green_color).unwrap()
        }
        select_mode::QMode::Local => todo!(),
        select_mode::QMode::Manual => {
            change_svg_prop::fill(element, MaterialTheme::extended_color_yellow_color).unwrap()
        }
        select_mode::QMode::Oos => todo!(),
    }
}

pub fn mode_text(element: &SvgElement, mode: select_mode::QMode) {
    match mode {
        select_mode::QMode::Auto => {
            change_svg_prop::text_content(element, "A").unwrap();
            change_svg_prop::text_color(element, MaterialTheme::extended_color_green_on_color)
                .unwrap()
        }
        select_mode::QMode::Local => todo!(),
        select_mode::QMode::Manual => {
            change_svg_prop::text_content(element, "P").unwrap();
            change_svg_prop::text_color(element, MaterialTheme::extended_color_yellow_on_color)
                .unwrap()
        }
        select_mode::QMode::Oos => todo!(),
    }
}
