use leptos::prelude::*;

/// Вид кнопки
#[allow(missing_docs)]
pub enum IconButtonKind {
    Icon,
    FilledIcon,
    FilledTonalIcon,
    OutlinedIcon,
}

#[component]
pub fn IconButton(
    /// Вид кнопки
    kind: IconButtonKind,

    /// Событие нажатия
    clicked: impl Fn() + 'static,

    /// true = кнопка заблокирована
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,

    /// true = кнопка выбрана
    #[prop(default = MaybeSignal::from(false))]
    selected: MaybeSignal<bool>,

    /// Работает в режиме переключения
    #[prop(default = false)]
    toggle: bool,

    children: Children,
) -> impl IntoView {
    match kind {
        IconButtonKind::Icon => view! {
            <md-icon-button
                on:click=move |_| (clicked)()
                disabled=move || disabled.get()
                selected=move || selected.get()
                toggle=toggle
            >
                {children()}
            </md-icon-button>
        }
        .into_any(),

        IconButtonKind::FilledIcon => view! {
            <md-filled-icon-button
                on:click=move |_| (clicked)()
                disabled=move || disabled.get()
                selected=move || selected.get()
                toggle=toggle
            >
                {children()}
            </md-filled-icon-button>
        }
        .into_any(),

        IconButtonKind::FilledTonalIcon => view! {
            <md-filled-tonal-icon-button
                on:click=move |_| (clicked)()
                disabled=move || disabled.get()
                selected=move || selected.get()
                toggle=toggle
            >
                {children()}
            </md-filled-tonal-icon-button>
        }
        .into_any(),

        IconButtonKind::OutlinedIcon => view! {
            <md-outlined-icon-button
                on:click=move |_| (clicked)()
                disabled=move || disabled.get()
                selected=move || selected.get()
                toggle=toggle
            >
                {children()}
            </md-outlined-icon-button>
        }
        .into_any(),
    }
}
