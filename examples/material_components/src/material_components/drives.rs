use leptos::prelude::*;
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
    let (motor_status, _) = create_signal_from_msg!("Custom-m1_status");
    let (_, motor_command) = create_signal_from_msg!("Custom-m1_command");

    let (valve_analog_status, _) = create_signal_from_msg!("Custom-valve_analog_status");
    let (_, valve_analog_command) = create_signal_from_msg!("Custom-valve_analog_command");

    let (valve_status, _) = create_signal_from_msg!("Custom-valve_hmi_status");
    let (_, valve_command) = create_signal_from_msg!("Custom-valve_hmi_command");

    let (fpt, fpt_set) = signal(String::from(""));

    let svg_file = include_str!("../../schemas/drives.svg");
    let svg_input = vec![
        SvgInput::plc_drives_motor("motor", motor_status),
        SvgInput::plc_drives_valve_analog("valve_analog", valve_analog_status),
        SvgInput::plc_drives_valve("valve", valve_status),
    ];
    let svg_output = SvgOutput {
        ids: ["motor", "valve_analog", "valve"]
            .iter()
            .map(|id| id.to_string())
            .collect(),
        callback: move |id| match id {
            "motor" => fpt_set.set("motor".into()),
            "valve_analog" => fpt_set.set("valve_analog".into()),
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
            title = "Двигатель"
            hmi_status = motor_status
            hmi_command = motor_command
            visible = Signal::derive(move || fpt.get() == "motor")
            on_close = close_fpt
        />

        <drives::ValveAnalog
            title = "Клапан аналоговый"
            hmi_status = valve_analog_status
            hmi_command = valve_analog_command
            visible = Signal::derive(move || fpt.get() == "valve_analog")
            on_close = close_fpt
        />

        <drives::Valve
            title = "Клапан"
            hmi_status = valve_status
            hmi_command = valve_command
            visible = Signal::derive(move || fpt.get() == "valve")
            on_close = close_fpt
        />

    }
}
