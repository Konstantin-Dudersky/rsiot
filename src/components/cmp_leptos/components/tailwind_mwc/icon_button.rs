use leptos::*;

/// Вид кнопки
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum IconButtonKind {
    Standard,
    StandardToggle,
    Filled,
    FilledToggle,
    FilledTonal,
    FilledTonalToggle,
    Outlined,
    OutlinedToggle,
}

#[component]
pub fn IconButton<FIcon, IVIcon>(
    /// Вид кнопки
    #[prop(default=IconButtonKind::Filled)]
    kind: IconButtonKind,

    /// Иконка
    ///
    /// Пример:
    ///
    /// ```html
    /// ||view!{  <span class="iconify material-symbols--play-arrow-rounded"></span> }
    /// ```
    icon: FIcon,

    /// true = кнопка заблокирована
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,

    /// true = кнопка выбрана
    #[prop(default = MaybeSignal::from(false))]
    selected: MaybeSignal<bool>,

    /// Событие нажатия
    on_click: impl Fn() -> () + 'static,
) -> impl IntoView
where
    FIcon: Fn() -> IVIcon,
    IVIcon: IntoView,
{
    view! {
        <button
            // class="h-10 w-10 rounded-full flex flex-row justify-center items-center"
            class="h-10 w-10 rounded-full p-2"
            class=("text-on-primary", move || matches!(kind, IconButtonKind::Filled))
            class=("bg-primary", move || matches!(kind, IconButtonKind::Filled))
            on:click = move |_| (on_click)()
        >
            // <div class="h-6 w-6">
                { icon() }
            // </div>
        </button>
    }
}
