use leptos::*;

use super::FilledButton;

#[component]
pub fn Motor(
    man_start: impl Fn() -> () + 'static,
    man_stop: impl Fn() -> () + 'static,
) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div>
                <p> Motor </p>
            </div>

            <div class="flex flex-row items-center my-4">
                <div class="grow">Состояние</div>

                <div>
                    <p class="p-2 bg-custom-color1-color rounded-sm text-custom-color1-on-color">
                        Работа
                    </p>
                </div>

                <div>
                    <FilledButton
                        clicked=man_start
                    >
                        <md-icon slot="icon">play_arrow</md-icon> Пуск
                    </FilledButton>
                </div>
                <div class="pl-4">
                    <FilledButton
                    clicked=man_stop
                    >
                        <md-icon slot="icon">stop</md-icon> Стоп
                    </FilledButton>
                </div>
            </div>
        </div>
    }
}
