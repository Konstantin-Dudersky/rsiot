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

    let (fpt, fpt_set) = create_signal(String::from(""));

    let svg_file = include_str!("../../schemas/drives.svg");
    let svg_input = vec![SvgInput::plc_drives_motor("M1", m1_status)];
    let svg_output = SvgOutput {
        ids: ["M1"].iter().map(|id| id.to_string()).collect(),
        callback: move |id| match id {
            "M1" => fpt_set.set("M1".into()),
            _ => (),
        },
    };

    view! {
        <SvgDynamic
            file=svg_file
            svg_input=svg_input
            svg_output=svg_output
        />

        <drives::Motor
            title = "Двигатель M1"
            hmi_status = m1_status
            hmi_command = move |command| m1_command.set(command)
            visible = move || fpt.get() == "M1"
            on_close = move || fpt_set.set("".into())
        />

    }
}
