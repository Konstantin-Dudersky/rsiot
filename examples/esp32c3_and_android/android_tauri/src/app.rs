use leptos::*;

use rsiot::{components::cmp_leptos::create_signal_from_msg, message::*};

use message::*;

#[component]
pub fn App() -> impl IntoView {
    let (gpio0_button, _) = create_signal_from_msg!("Custom-Gpio0Button");
    let (analog_pin2, _) = create_signal_from_msg!("Custom-AnalogPin2");

    let (_, set_relay_state) = create_signal_from_msg!("Custom-SetRelayState");

    view! {
        <main class="container mx-auto">
            <div class="flex flex-row">
                <div class="basis-1/4">Вход GPIO0</div>
                <div class="basis-3/4">{ move || gpio0_button.get() }</div>
            </div>
            <div class="flex flex-row">
                <div class="basis-3/12">Выход GPIO</div>
                <div class="basis-4/12">
                    <button
                        class="bg-white opacity-10"
                        on:click = move |_| {set_relay_state.set(true)}
                    >
                        Включить
                    </button>
                </div>
                <div class="basis-4/12">
                    <button
                        on:click = move |_| {set_relay_state.set(false)}
                    >
                        Отключить
                    </button>
                </div>
            </div>
            <div class="flex flex-row">
                <div class="basis-1/4">Аналоговое значение</div>
                <div class="basis-3/4">{ move || analog_pin2.get() }</div>
            </div>
        </main>
    }
}
