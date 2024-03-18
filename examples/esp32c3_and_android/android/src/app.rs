use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use rsiot::{components::cmp_leptos::*, message::*};

use message::*;

#[component]
pub fn App() -> impl IntoView {
    let (button, _) = create_signal_from_msg!("Custom-BootButton");

    let (_, relay_on) = create_signal_from_msg!("Custom-SetRelayState");

    view! {
        <main class="container">
            Состояние кнопки: { move || button.get() }

            <br/>

            <button on:click=move |_| relay_on.set(true)> Включить </button>
            <button on:click=move |_| relay_on.set(false)> Отключить </button>
        </main>
    }
}
