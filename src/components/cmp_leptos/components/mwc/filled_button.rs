use leptos::*;

/// Добавить в `input.js`:
///
/// ```js
/// import "@material/web/button/filled-button.js";
/// ```
#[component]
pub fn FilledButton<FClicked>(
    /// Событие нажатия
    clicked: FClicked,

    #[prop(default = false.into())] disabled: MaybeSignal<bool>,

    /// Текст кнопки
    children: Children,
) -> impl IntoView
where
    FClicked: Fn() -> () + 'static,
{
    view! {
        <md-filled-button
            on:click= move |_| (clicked)()
            disabled=move || disabled.get()
        >
            { children() }
        </md-filled-button>
    }
}
