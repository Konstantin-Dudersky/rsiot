use leptos::*;
use rsiot::{
    components::cmp_leptos::{
        components::svg_dynamic::{SvgDynamic, SvgInput},
        create_signal_from_msg,
    },
    logging::configure_logging,
    message::*,
};

use crate::Message;

#[component]
pub fn App() -> impl IntoView {
    configure_logging("info");

    let (counter, _) = create_signal_from_msg!(Message::U16_0_100);

    let svg_input = vec![SvgInput::rect_y("rect1", move || counter.get().value as f64)];

    view! {
        <SvgDynamic file=include_str!("../schemas/scheme1.svg") svg_input=svg_input/>
    }
}
