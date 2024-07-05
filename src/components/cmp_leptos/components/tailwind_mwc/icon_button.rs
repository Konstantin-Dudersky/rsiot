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
    /// || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
    /// ```
    icon: FIcon,

    /// true = кнопка заблокирована
    #[prop(default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,

    /// true = кнопка выбрана. Для типов xxxToggle
    #[prop(default = MaybeSignal::Static(false))]
    toggled: MaybeSignal<bool>,

    /// Событие нажатия
    on_click: impl Fn() -> () + 'static,
) -> impl IntoView
where
    FIcon: Fn() -> IVIcon,
    IVIcon: IntoView,
{
    view! {
        <button
            class="h-10 w-10 rounded-full relative"
            disabled=move || disabled.get()
            on:click = move |_| (on_click)()
        >
            <div
                id="container"
                class="absolute left-0 top-0 z-0 h-full w-full rounded-full border-outline"

                class=("bg-transparent", move || matches!(kind, IconButtonKind::Standard))
                class=("bg-transparent", move || matches!(kind, IconButtonKind::StandardToggle))

                class=("bg-primary", move || matches!(kind, IconButtonKind::Filled))
                class=("bg-surface-variant", move || matches!(kind, IconButtonKind::FilledToggle) && !toggled.get())
                class=("bg-primary", move || matches!(kind, IconButtonKind::FilledToggle) && toggled.get())

                class=("bg-secondary-container", move || matches!(kind, IconButtonKind::FilledTonal))
                class=("bg-surface-variant", move || matches!(kind, IconButtonKind::FilledTonalToggle) && !toggled.get())
                class=("bg-secondary-container", move || matches!(kind, IconButtonKind::FilledTonalToggle) && toggled.get())

                class=("bg-transparent", move || matches!(kind, IconButtonKind::Outlined))
                class=("bg-transparent", move || matches!(kind, IconButtonKind::OutlinedToggle) && !toggled.get())
                class=("bg-inverse-surface", move || matches!(kind, IconButtonKind::OutlinedToggle) && toggled.get())

                class=("border", move || matches!(kind, IconButtonKind::Outlined))
                class=("border", move || matches!(kind, IconButtonKind::OutlinedToggle) && !toggled.get())
                class=("border-0", move || matches!(kind, IconButtonKind::OutlinedToggle) && toggled.get())

                class=("invisible", move || disabled.get())
            >
            </div>

            <div
                id="container-disabled"
                class="absolute left-0 top-0 z-0 h-full w-full rounded-full border-outline opacity-[.12]"

                class=("bg-transparent", move || matches!(kind, IconButtonKind::Standard))
                class=("bg-transparent", move || matches!(kind, IconButtonKind::StandardToggle))

                class=("bg-on-surface", move || matches!(kind, IconButtonKind::Filled))
                class=("bg-on-surface", move || matches!(kind, IconButtonKind::FilledToggle))

                class=("bg-on-surface", move || matches!(kind, IconButtonKind::FilledTonal))
                class=("bg-on-surface", move || matches!(kind, IconButtonKind::FilledTonalToggle))

                class=("bg-transparent", move || matches!(kind, IconButtonKind::Outlined))
                class=("bg-transparent", move || matches!(kind, IconButtonKind::OutlinedToggle))

                class=("border", move || matches!(kind, IconButtonKind::Outlined))
                class=("border", move || matches!(kind, IconButtonKind::OutlinedToggle))

                class=("invisible", move || !disabled.get())
            >

            </div>

            <div
                id="icon"
                class="absolute left-2 top-2 w-6 h-6 z-10"

                class=("text-on-surface-variant", move || matches!(kind, IconButtonKind::Standard))
                class=("text-on-surface-variant", move || matches!(kind, IconButtonKind::StandardToggle) && !toggled.get())
                class=("text-primary", move || matches!(kind, IconButtonKind::StandardToggle) && toggled.get())

                class=("text-on-primary", move || matches!(kind, IconButtonKind::Filled))
                class=("text-primary", move || matches!(kind, IconButtonKind::FilledToggle) && !toggled.get())
                class=("text-on-primary", move || matches!(kind, IconButtonKind::FilledToggle) && toggled.get())

                class=("text-on-secondary-container", move || matches!(kind, IconButtonKind::FilledTonal))
                class=("text-on-surface-variant", move || matches!(kind, IconButtonKind::FilledTonalToggle) && !toggled.get())
                class=("text-on-secondary-container", move || matches!(kind, IconButtonKind::FilledTonalToggle) && toggled.get())

                class=("text-on-surface-variant", move || matches!(kind, IconButtonKind::Outlined))
                class=("text-on-surface-variant", move || matches!(kind, IconButtonKind::OutlinedToggle) && !toggled.get())
                class=("text-inverse-on-surface", move || matches!(kind, IconButtonKind::OutlinedToggle) && toggled.get())

                class=("invisible", move || disabled.get())

            >
                { icon() }
            </div>

            <div
                id="icon-disabled"
                class="absolute left-2 top-2 w-6 h-6 z-10 opacity-[.38] text-on-surface"

                class=("invisible", move || !disabled.get())

            >
                { icon() }
            </div>

            <div id="hover" class="absolute left-0 top-0 z-20 h-full w-full rounded-full bg-on-surface-variant opacity-0 hover:opacity-10"
                class=("invisible", move || disabled.get())
            >

            </div>

        </button>
    }
}
