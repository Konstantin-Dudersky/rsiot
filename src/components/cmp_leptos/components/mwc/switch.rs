use leptos::prelude::*;

/// Переключатель
///
/// [Документация](https://material-web.dev/components/switch/)
///
/// Добавить в `input.js`:
///
/// ```js
/// import "@material/web/switch/switch";
/// ```
#[component]
pub fn Switch(
    /// Состояние переключателя
    #[prop(into)]
    status: Signal<bool>,

    /// Событие переключения
    clicked: WriteSignal<()>,
) -> impl IntoView {
    view! {
        <md-switch
            selected=move|| status.get()
            on:click= move |_| clicked.set(())
        >
        </md-switch>
    }
}
