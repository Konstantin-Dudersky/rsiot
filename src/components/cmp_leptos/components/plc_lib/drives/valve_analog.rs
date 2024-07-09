use leptos::*;

use crate::components::cmp_plc::plc::library::drives::{
    select_mode,
    valve_analog::{IHmiCommand, QHmiStatus},
};

use super::{
    super::super::tailwind_mwc::{Dialog, InputType, TextField},
    shared::SelectMode,
};

#[component]
pub fn ValveAnalog(
    /// Заголовок
    title: &'static str,

    /// Состояние
    #[prop(into)]
    hmi_status: Signal<QHmiStatus>,

    /// Управление
    hmi_command: WriteSignal<IHmiCommand>,

    /// Видимость
    #[prop(into)]
    visible: Signal<bool>,

    /// Нажатие кнопки "Закрыть"
    on_close: impl Fn() + 'static + Copy,
) -> impl IntoView {
    view! {
        <Dialog
            visible=move || visible.get()
            headline=move || view! { {title} }
            content=move || {
                view! { <Content hmi_status=hmi_status hmi_command=hmi_command/> }
            }
            actions=move || {
                view! { <button on:click=move |_| { on_close() }>Закрыть</button> }
            }
        />
    }
}

#[component]
fn Content(
    /// Состояние
    #[prop(into)]
    hmi_status: Signal<QHmiStatus>,

    /// Управление
    hmi_command: WriteSignal<IHmiCommand>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">

            // Режим работы ------------------------------------------------------------------------
            <SelectMode
                mode = move || hmi_status.get().mode
                hmi_permission_mode_man = move || hmi_status.get().hmi_permission.mode_man
                hmi_permission_mode_auto = move || hmi_status.get().hmi_permission.mode_auto
                hmi_permission_mode_local = move || hmi_status.get().hmi_permission.mode_local
                hmi_permission_mode_oos = move || hmi_status.get().hmi_permission.mode_oos
                on_hmi_command = move |hc| {
                    let hc = match hc {
                        select_mode::IHmiCommand::no_command => IHmiCommand::no_command,
                        select_mode::IHmiCommand::mode_man => IHmiCommand::mode_man,
                        select_mode::IHmiCommand::mode_auto => IHmiCommand::mode_auto,
                        select_mode::IHmiCommand::mode_local => IHmiCommand::mode_local,
                        select_mode::IHmiCommand::mode_oos => IHmiCommand::mode_oos,
                    };
                    hmi_command.set(hc);
                }
            />

            // Задание работы ----------------------------------------------------------------------
            <div class="flex flex-row items-center">
                <p class="grow">
                    Задание
                </p>

                <div class="w-32">
                    <TextField
                        value = move || hmi_status.get().mv.to_string()
                        on_keyup_enter = move |mv| {
                            let mv = mv.parse::<f64>().unwrap();
                            hmi_command.set(IHmiCommand::mv_hmi(mv));
                        }
                        input_type = InputType::Number
                    />
                </div>

            </div>

            // Факт. открытие ----------------------------------------------------------------------
            <div class="flex flex-row items-center">
                <p class="grow">
                    Факт. открытие
                </p>

                <div class="w-32">
                    <TextField
                        value = move || hmi_status.get().rbk.to_string()
                        on_keyup_enter = move |_| ()
                        input_type = InputType::Number
                    />
                </div>

            </div>

        </div>

    }
}
