use leptos::prelude::*;

#[allow(missing_docs)]
pub enum ButtonKind {
    Elavated,
    Filled,
    FilledTonal,
    Outlined,
    Text,
}

#[component]
pub fn Button<FIcon, IVIcon>(
    /// Вид кнопки
    #[prop(default = ButtonKind::Filled)]
    button_kind: ButtonKind,

    /// Иконка
    ///
    /// Пример:
    ///
    /// ```html
    /// ||view!{  <span class="iconify material-symbols--play-arrow-rounded w-5 h-5"></span> }
    /// ```
    icon: FIcon,

    /// Текст кнопки
    #[prop(default = "Кнопка")]
    text: &'static str,

    /// Событие нажатия
    on_click: impl Fn() + 'static,

    /// true = кнопка заблокирована
    #[prop(default = Signal::derive(|| false))]
    disabled: Signal<bool>,
) -> impl IntoView
where
    FIcon: Fn() -> IVIcon,
    IVIcon: IntoView,
{
    match button_kind {
        ButtonKind::Elavated => todo!(),
        ButtonKind::Filled => view! {
            <button
                on:click = move |_| (on_click)()
                disabled=move || disabled.get()
                class="text-on-primary disabled:text-surface bg-primary disabled:bg-on-surface h-10 rounded-full disabled:opacity-10"
            >
                <div class="flex h-full flex-row items-center pl-4 pr-6">
                    {icon()}
                    <div class="pl-2">
                        { text }
                    </div>
                </div>
            </button>
        },
        ButtonKind::FilledTonal => todo!(),
        ButtonKind::Outlined => todo!(),
        ButtonKind::Text => todo!(),
    }
}

// TODO - как задать opacity для текста отдельно от кнопки? Должно быть opacity-40
