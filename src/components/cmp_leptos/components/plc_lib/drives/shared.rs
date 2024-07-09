use leptos::*;

use crate::components::cmp_plc::plc::library::drives::select_mode::{IHmiCommand, QMode};

use super::super::super::tailwind_mwc::{Button, IconButton, IconButtonKind};

#[component]
pub fn SelectMode(
    #[prop(into)] mode: Signal<QMode>,
    #[prop(into)] hmi_permission_mode_man: Signal<bool>,
    #[prop(into)] hmi_permission_mode_auto: Signal<bool>,
    #[prop(into)] hmi_permission_mode_local: Signal<bool>,
    #[prop(into)] hmi_permission_mode_oos: Signal<bool>,

    on_hmi_command: impl Fn(IHmiCommand) + 'static + Copy,
) -> impl IntoView {
    let (visible_mode, visible_mode_set) = create_signal(false);

    view! {
        <div class="flex flex-row items-center gap-4">
            <div class="grow">Режим работы</div>

            <div>
                <Show when=move || mode.get() == QMode::Auto>
                    <p class="p-2 rounded-sm bg-green-color-container text-green-on-color-container">
                        Авто
                    </p>
                </Show>

                <Show when=move || mode.get() == QMode::Local>
                    <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                        Местный
                    </p>
                </Show>

                <Show when=move || mode.get() == QMode::Manual>
                    <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                        Ручной
                    </p>
                </Show>

                <Show when=move || mode.get() == QMode::Oos>
                    <p class="p-2 rounded-sm bg-yellow-color-container text-green-on-color-container">
                        Выведен
                    </p>
                </Show>
            </div>

            <div>
                <IconButton
                    kind=IconButtonKind::OutlinedToggle
                    icon=|| view! { <span class="iconify material-symbols--more-horiz h-6 w-6"></span> }
                    disabled=MaybeSignal::derive(move || {
                        !hmi_permission_mode_man.get()
                            && !hmi_permission_mode_auto.get()
                            && !hmi_permission_mode_local.get()
                            && !hmi_permission_mode_oos.get()
                    })
                    toggled=MaybeSignal::derive(move || visible_mode.get())
                    on_click= move || visible_mode_set.update(|v| *v = !*v)
                />
            </div>
        </div>

        <Show when=move || visible_mode.get()>
            <div class="flex flex-wrap gap-2 my-4">

                <div>
                    <Button
                        on_click=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            on_hmi_command(IHmiCommand::mode_auto)
                        }
                        icon=||view!{
                            <span class="iconify material-symbols--play-arrow-rounded w-5 h-5"></span>
                        }
                        text="Авто"
                    />
                </div>
                <div>
                    <Button
                        on_click=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            on_hmi_command(IHmiCommand::mode_man)
                        }
                        icon=||view!{
                            <span class="iconify material-symbols--pan-tool-rounded w-5 h-5"></span>
                        }
                        text="Ручной"
                    />
                </div>
                <div>
                    <Button
                        on_click=move || {
                            visible_mode_set.update(|v| *v = !*v);
                            on_hmi_command(IHmiCommand::mode_local)
                        }
                        icon=||view!{
                            <span class="iconify material-symbols--switch-rounded w-5 h-5"></span>
                        }
                        text="Местный"
                    />
                </div>
            </div>
        </Show>
    }
}
