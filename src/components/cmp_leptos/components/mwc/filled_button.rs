use leptos::*;

/// Добавить в `input.js`:
///
/// ```js
/// import "@material/web/button/filled-button.js";
/// ```
#[component]
pub fn FilledButton(
    /// Событие нажатия
    clicked: impl Fn() + 'static,

    /// true = кнопка заблокирована
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,

    /// Текст кнопки
    children: Children,
) -> impl IntoView {
    view! {
        <md-filled-button on:click=move |_| (clicked)() disabled=move || disabled.get()>
            {children()}
        </md-filled-button>
    }
}
