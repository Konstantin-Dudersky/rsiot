use leptos::*;

use crate::components::cmp_plc::plc::library::drives::valve::{
    IHmiCommand, QHmiStatus, QMode, QState,
};

use super::{Dialog, FilledButton, IconButton, IconButtonKind};

#[component]
pub fn Valve(
    /// Заголовок
    title: &'static str,

    hmi_command: impl Fn(IHmiCommand) -> () + 'static + Copy,
    #[prop(into)] hmi_status: Signal<QHmiStatus>,
) -> impl IntoView {
    let (open_state_dialog, open_state_dialog_set) = create_signal(());
    let (open_mode_dialog, open_mode_dialog_set) = create_signal(());

    view! {
        <div class="flex flex-col">
            <div>
                <p>{title}</p>
            </div>

            // Команда -----------------------------------------------------------------------------

            <div class="flex flex-row items-center my-4">
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

                <div class="pl-4">
                    <IconButton
                        kind=IconButtonKind::OutlinedIcon
                        clicked=move || open_state_dialog_set.set(())
                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.man_start
                                && !hmi_status.get().hmi_permission.man_stop
                        })
                    >

                        <md-icon>more_horiz</md-icon>
                    </IconButton>
                </div>
            </div>

            // Режим работы ------------------------------------------------------------------------

            <div class="flex flex-row items-center my-4">
                <div class="grow">Режим работы</div>

                <div>
                    <Show when=move || hmi_status.get().mode == QMode::Auto>
                        <p class="p-2 rounded-sm bg-custom-color1-color text-custom-color1-on-color">
                            Авто
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Local>
                        <p class="p-2 rounded-sm bg-custom-color3-color text-custom-color1-on-color">
                            Местный
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Manual>
                        <p class="p-2 rounded-sm bg-custom-color2-color text-custom-color1-on-color">
                            Ручной
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Oos>
                        <p class="p-2 rounded-sm bg-custom-color3-color text-custom-color1-on-color">
                            Выведен
                        </p>
                    </Show>
                </div>

                <div class="pl-4">
                    <IconButton
                        kind=IconButtonKind::OutlinedIcon
                        clicked=move || open_mode_dialog_set.set(())
                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.man_mode
                                && !hmi_status.get().hmi_permission.auto_mode
                                && !hmi_status.get().hmi_permission.local_mode
                                && !hmi_status.get().hmi_permission.oos_mode
                        })
                    >

                        <md-icon>more_horiz</md-icon>
                    </IconButton>
                </div>
            </div>

        </div>

        <Dialog
            headline=|| view! { Команда }
            content=|| {
                view! {
                    <form method="dialog">
                        <div class="flex flex-wrap gap-2">
                            <div>
                                <FilledButton
                                    clicked=move || hmi_command(IHmiCommand::OpenMan)
                                    disabled=MaybeSignal::derive(move || {
                                        !hmi_status.get().hmi_permission.man_start
                                    })
                                >

                                    <md-icon slot="icon">expand_all</md-icon>
                                    Открыть
                                </FilledButton>
                            </div>
                            <div>
                                <FilledButton
                                    clicked=move || hmi_command(IHmiCommand::CloseMan)
                                    disabled=MaybeSignal::derive(move || {
                                        !hmi_status.get().hmi_permission.man_stop
                                    })
                                >

                                    <md-icon slot="icon">collapse_all</md-icon>
                                    Закрыть
                                </FilledButton>
                            </div>
                        </div>
                    </form>
                }
            }

            actions=|| {
                view! {
                    <form method="dialog">
                        <FilledButton clicked=|| ()>
                            <md-icon slot="icon">close</md-icon>
                            Закрыть
                        </FilledButton>
                    </form>
                }
            }

            open=open_state_dialog
        />

        <Dialog
            headline=|| view! { Режим работы }
            content=|| {
                view! {
                    <form method="dialog">
                        <div class="flex flex-wrap gap-2">

                            <div>
                                <FilledButton clicked=move || hmi_command(IHmiCommand::AutoMode)>
                                    <md-icon slot="icon">autoplay</md-icon>
                                    Авто
                                </FilledButton>
                            </div>
                            <div>
                                <FilledButton clicked=move || hmi_command(IHmiCommand::ManMode)>
                                    <md-icon slot="icon">pan_tool</md-icon>
                                    Ручной
                                </FilledButton>
                            </div>
                            <div>
                                <FilledButton clicked=move || hmi_command(IHmiCommand::LocalMode)>
                                    <md-icon slot="icon">switch</md-icon>
                                    Местный
                                </FilledButton>
                            </div>
                        </div>
                    </form>
                }
            }

            actions=|| {
                view! {
                    <form method="dialog">
                        <FilledButton clicked=|| ()>
                            <md-icon slot="icon">close</md-icon>
                            Закрыть
                        </FilledButton>
                    </form>
                }
            }

            open=open_mode_dialog
        />
    }
}
