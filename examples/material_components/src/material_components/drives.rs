use leptos::*;
use rsiot::{
    components::cmp_leptos::{
        components::{
            plc_lib::drives,
            svg_dynamic::{SvgDynamic, SvgInput, SvgOutput},
        },
        create_signal_from_msg,
    },
    message::*,
};

use crate::messages::*;

#[component]
pub fn Drives() -> impl IntoView {
    let (m1_status, _) = create_signal_from_msg!("Custom-m1_status");
    let (_, m1_command) = create_signal_from_msg!("Custom-m1_command");

    let (v1_status, _) = create_signal_from_msg!("Custom-valve_analog_status");
    let (_, v1_command) = create_signal_from_msg!("Custom-valve_analog_command");

    let (valve_status, _) = create_signal_from_msg!("Custom-valve_hmi_status");
    let (_, valve_command) = create_signal_from_msg!("Custom-valve_hmi_command");

    let (fpt, fpt_set) = create_signal(String::from(""));

    let svg_file = include_str!("../../schemas/drives.svg");
    let svg_input = vec![
        SvgInput::plc_drives_motor("M1", m1_status),
        SvgInput::plc_drives_valve_analog("V1", v1_status),
        SvgInput::plc_drives_valve("valve", valve_status),
    ];
    let svg_output = SvgOutput {
        ids: ["M1", "V1", "valve"]
            .iter()
            .map(|id| id.to_string())
            .collect(),
        callback: move |id| match id {
            "M1" => fpt_set.set("M1".into()),
            "V1" => fpt_set.set("valve_analog".into()),
            "valve" => fpt_set.set("valve".into()),
            _ => (),
        },
    };

    let close_fpt = move || fpt_set.set("".into());

    view! {
        <SvgDynamic
            file=svg_file
            svg_input=svg_input
            svg_output=svg_output
        />

        <drives::Motor
            title = "Двигатель M1"
            hmi_status = m1_status
            hmi_command = m1_command
            visible = move || fpt.get() == "M1"
            on_close = close_fpt
        />

        <drives::ValveAnalog
            title = "Клапан аналоговый"
            hmi_status = v1_status
            hmi_command = v1_command
            visible = move || fpt.get() == "valve_analog"
            on_close = close_fpt
        />

        <drives::Valve
            title = "Клапан"
            hmi_status = valve_status
            hmi_command = valve_command
            visible = move || fpt.get() == "valve"
            on_close = close_fpt
        />

    }
}
