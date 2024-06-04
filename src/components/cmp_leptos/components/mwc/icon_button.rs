use leptos::*;

/// Вид кнопки
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
    clicked: impl Fn() -> () + 'static,

    /// true = кнопка заблокирована
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,

    children: Children,
) -> impl IntoView {
    match kind {
        IconButtonKind::Icon => {
            view! {
                <md-icon-button on:click=move |_| (clicked)() disabled=move || disabled.get()>
                    {children()}
                </md-icon-button>
            }
        }

        IconButtonKind::FilledIcon => {
            view! {
                <md-filled-icon-button
                    on:click=move |_| (clicked)()
                    disabled=move || disabled.get()
                >
                    {children()}
                </md-filled-icon-button>
            }
        }

        IconButtonKind::FilledTonalIcon => {
            view! {
                <md-filled-tonal-icon-button
                    on:click=move |_| (clicked)()
                    disabled=move || disabled.get()
                >
                    {children()}
                </md-filled-tonal-icon-button>
            }
        }

        IconButtonKind::OutlinedIcon => {
            view! {
                <md-outlined-icon-button
                    on:click=move |_| (clicked)()
                    disabled=move || disabled.get()
                >
                    {children()}
                </md-outlined-icon-button>
            }
        }
    }
}
