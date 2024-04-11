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
            <div class="flex flex-row justify-around items-center mt-8">
                <div class="basis-4/12">
                    <p>GPIO 0 (вход)</p>
                </div>
                <div class="basis-8/12">{ move || gpio0_button.get() }</div>
            </div>
            <div class="flex flex-row items-center mt-8">
                <div class="basis-4/12">
                    <p>GPIO 1 (выход)</p>
                </div>
                <div class="basis-4/12">
                    <button type="button" class="py-2.5 px-3.5 text-sm font-semibold rounded-md shadow-sm bg-secondaryContainer text-onSecondaryContainer focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
                        on:click = move |_| {set_relay_state.set(true)}
                    >Включить
                    </button>
                </div>
                <div class="basis-4/12">
                    <button type="button" class="py-2.5 px-3.5 text-sm font-semibold rounded-md shadow-sm bg-secondaryContainer text-onSecondaryContainer focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
                        on:click = move |_| {set_relay_state.set(false)}
                    >
                        Отключить
                    </button>
                </div>
            </div>
            <div class="flex flex-row justify-around items-center mt-8">
                <div class="basis-4/12">
                    <p>GPIO 2 (вход ADC)</p>
                </div>
                <div class="basis-8/12">{ move || analog_pin2.get() }</div>
            </div>
        </main>
    }
}
