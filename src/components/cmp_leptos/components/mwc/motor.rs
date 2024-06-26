use leptos::*;

use crate::components::cmp_plc::plc::library::drives::motor::{
    IHmiCommand, QHmiStatus, QMode, QState,
};

use super::{FilledButton, IconButton, IconButtonKind};

#[component]
pub fn Motor(
    /// Состояние
    #[prop(into)]
    hmi_status: Signal<QHmiStatus>,

    /// Управление
    hmi_command: impl Fn(IHmiCommand) -> () + 'static + Copy,
) -> impl IntoView {
    let (visible_state, visible_state_set) = create_signal(false);
    let (visible_mode, visible_mode_set) = create_signal(false);

    view! {
        <div class="flex flex-col gap-2">
            // Команда -----------------------------------------------------------------------------

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
                        kind=IconButtonKind::OutlinedIcon
                        clicked=move || visible_state_set.update(|v| *v = !*v)
                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.man_start
                                && !hmi_status.get().hmi_permission.man_stop
                        })

                        selected=MaybeSignal::derive(move || visible_state.get())
                        toggle=true
                    >

                        <md-icon>more_horiz</md-icon>
                        <md-icon slot="selected">close</md-icon>
                    </IconButton>
                </div>
            </div>

            <Show when=move || visible_state.get()>
                <div class="flex flex-wrap gap-2">
                    <div>
                        <FilledButton
                            clicked=move || {
                                visible_state_set.update(|v| *v = !*v);
                                hmi_command(IHmiCommand::ManStart)
                            }

                            disabled=MaybeSignal::derive(move || {
                                !hmi_status.get().hmi_permission.man_start
                            })
                        >

                            <md-icon slot="icon">play_arrow</md-icon>
                            Пуск
                        </FilledButton>
                    </div>
                    <div>
                        <FilledButton
                            clicked=move || {
                                visible_state_set.update(|v| *v = !*v);
                                hmi_command(IHmiCommand::ManStop)
                            }

                            disabled=MaybeSignal::derive(move || {
                                !hmi_status.get().hmi_permission.man_stop
                            })
                        >

                            <md-icon slot="icon">stop</md-icon>
                            Стоп
                        </FilledButton>
                    </div>
                </div>
            </Show>

            <md-divider></md-divider>

            // Режим работы ------------------------------------------------------------------------

            <div class="flex flex-row items-center gap-4">
                <div class="grow">Режим работы</div>

                <div>
                    <Show when=move || hmi_status.get().mode == QMode::Auto>
                        <p class="p-2 rounded-sm bg-green-color-container text-green-on-color-container">
                            Авто
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Local>
                        <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                            Местный
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Manual>
                        <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                            Ручной
                        </p>
                    </Show>

                    <Show when=move || hmi_status.get().mode == QMode::Oos>
                        <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                            Выведен
                        </p>
                    </Show>
                </div>

                <div>
                    <IconButton
                        kind=IconButtonKind::OutlinedIcon
                        clicked=move || visible_mode_set.update(|v| *v = !*v)
                        disabled=MaybeSignal::derive(move || {
                            !hmi_status.get().hmi_permission.mode_man
                                && !hmi_status.get().hmi_permission.mode_auto
                                && !hmi_status.get().hmi_permission.mode_local
                                && !hmi_status.get().hmi_permission.mode_oos
                        })

                        selected=MaybeSignal::derive(move || visible_mode.get())
                        toggle=true
                    >

                        <md-icon>more_horiz</md-icon>
                        <md-icon slot="selected">close</md-icon>
                    </IconButton>
                </div>
            </div>

            <Show when=move || visible_mode.get()>
                <div class="flex flex-wrap gap-2 my-4">

                    <div>
                        <FilledButton clicked=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            hmi_command(IHmiCommand::mode_auto)
                        }>
                            <md-icon slot="icon">autoplay</md-icon>
                            Авто
                        </FilledButton>
                    </div>
                    <div>
                        <FilledButton clicked=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            hmi_command(IHmiCommand::mode_man)
                        }>
                            <md-icon slot="icon">pan_tool</md-icon>
                            Ручной
                        </FilledButton>
                    </div>
                    <div>
                        <FilledButton clicked=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            hmi_command(IHmiCommand::mode_local)
                        }>
                            <md-icon slot="icon">switch</md-icon>
                            Местный
                        </FilledButton>
                    </div>
                </div>
            </Show>

        </div>
    }
}
