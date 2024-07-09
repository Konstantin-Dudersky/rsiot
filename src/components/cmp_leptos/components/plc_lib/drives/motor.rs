use leptos::*;

use crate::components::cmp_plc::plc::library::drives::{
    motor::{IHmiCommand, QHmiStatus, QState},
    select_mode,
};

use super::super::super::tailwind_mwc::{Button, Dialog, IconButton, IconButtonKind};

use super::shared::SelectMode;

#[component]
pub fn Motor(
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
            // State -------------------------------------------------------------------------------
            <State
                hmi_status=hmi_status
                hmi_command=hmi_command
            />

            <md-divider></md-divider>

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

        </div>
    }
}

#[component]
fn State(
    #[prop(into)] hmi_status: Signal<QHmiStatus>,

    /// Управление
    hmi_command: WriteSignal<IHmiCommand>,
) -> impl IntoView {
    let (visible_state, visible_state_set) = create_signal(false);

    view! {
        <div class="flex flex-row items-center gap-4">
            <div class="grow">Команда</div>

            <div>
                <Show when=move || hmi_status.get().state == QState::Stop>
                    <p class="p-2 rounded-sm bg-custom-color3-color text-custom-color1-on-color">
                        Стоп
                    </p>
                </Show>

                <Show when=move || hmi_status.get().state == QState::Start>
                    <p class="p-2 rounded-sm bg-custom-color1-color text-custom-color1-on-color">
                        Пуск
                    </p>
                </Show>
            </div>

            <div>
                <IconButton
                    kind=IconButtonKind::OutlinedToggle
                    icon=|| view! {
                        <span class="iconify material-symbols--more-horiz h-6 w-6"></span>
                    }
                    disabled=MaybeSignal::derive(move || {
                        !hmi_status.get().hmi_permission.man_start
                            && !hmi_status.get().hmi_permission.man_stop
                    })
                    toggled=MaybeSignal::derive(move || visible_state.get())
                    on_click= move || visible_state_set.update(|v| *v = !*v)
                />
            </div>
        </div>

        <Show when=move || visible_state.get()>
            <div class="flex flex-wrap gap-2">
                <div>
                    <Button
                        on_click=move || {
                            visible_state_set.update(|v| *v = !*v);
                            hmi_command.set(IHmiCommand::man_start)
                        }

                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.man_start
                        })

                        icon=||view!{
                            <span class="iconify material-symbols--play-arrow-rounded w-5 h-5"></span>
                        }

                        text="Пуск"
                    />
                </div>
                <div>
                    <Button
                        on_click=move || {
                            visible_state_set.update(|v| *v = !*v);
                            hmi_command.set(IHmiCommand::man_stop)
                        }

                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.man_stop
                        })

                        icon=||view!{
                            <span class="iconify material-symbols--stop-rounded w-5 h-5"></span>
                        }

                        text="Стоп"
                    />
                </div>
            </div>
        </Show>
    }
}
