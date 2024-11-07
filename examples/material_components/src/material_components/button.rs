use leptos::prelude::*;
use rsiot::components::cmp_leptos::components::tailwind_mwc::{Button, ButtonKind};
use tracing::info;

#[component]
pub fn ButtonView() -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 place-items-center">
            <div>
                ButtonKind
            </div>

            <div>
                Enabled
            </div>

            <div>
                Disabled
            </div>

            <div>
                ButtonKind::Filled
            </div>

            <div>
                <Button
                    button_kind = ButtonKind::Filled
                    icon = || ()
                    text = "Кнопка"
                    on_click = || info!("Clicked")
                    disabled = false.into()
                />

                <Button
                    button_kind = ButtonKind::Filled
                    icon = || view! { <span class="iconify material-symbols--play-arrow-rounded w-5 h-5"></span> }
                    text = "Кнопка"
                    on_click = || info!("Clicked")
                    disabled = false.into()
                />
            </div>

            <div>
                <Button
                    button_kind = ButtonKind::Filled
                    icon = || ()
                    text = "Кнопка"
                    on_click = || info!("Clicked")
                    disabled = true.into()
                />

                <Button
                    button_kind = ButtonKind::Filled
                    icon = || view! { <span class="iconify material-symbols--play-arrow-rounded w-5 h-5"></span> }
                    text = "Кнопка"
                    on_click = || info!("Clicked")
                    disabled = true.into()
                />
            </div>

        </div>
    }
}
