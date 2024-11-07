use leptos::prelude::*;

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
    #[prop(default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// true = кнопка выбрана. Для типов xxxToggle
    #[prop(default = Signal::derive(|| false))]
    toggled: Signal<bool>,

    /// Событие нажатия
    on_click: impl Fn() + 'static,
) -> impl IntoView
where
    FIcon: Fn() -> IVIcon,
    IVIcon: IntoView,
{
    let container_bg_transparent = move || match kind {
        IconButtonKind::Standard => true,
        IconButtonKind::StandardToggle => true,
        IconButtonKind::Outlined => true,
        IconButtonKind::OutlinedToggle => !toggled.get(),
        _ => false,
    };

    let container_bg_primary = move || match kind {
        IconButtonKind::Filled => true,
        IconButtonKind::FilledToggle => toggled.get(),
        _ => false,
    };

    let container_bg_surface_variant = move || match kind {
        IconButtonKind::FilledToggle => !toggled.get(),
        IconButtonKind::FilledTonalToggle => !toggled.get(),
        _ => false,
    };

    let container_bg_secondary_container = move || match kind {
        IconButtonKind::FilledTonal => true,
        IconButtonKind::FilledTonalToggle => toggled.get(),
        _ => false,
    };

    let container_border = move || match kind {
        IconButtonKind::Outlined => true,
        IconButtonKind::OutlinedToggle => !toggled.get(),
        _ => false,
    };

    let container_disabled_bg_transparent = move || {
        matches!(
            kind,
            IconButtonKind::Standard
                | IconButtonKind::StandardToggle
                | IconButtonKind::Outlined
                | IconButtonKind::OutlinedToggle
        )
    };

    let container_disabled_bg_on_surface = move || {
        matches!(
            kind,
            IconButtonKind::Filled
                | IconButtonKind::FilledToggle
                | IconButtonKind::FilledTonal
                | IconButtonKind::FilledTonalToggle
        )
    };

    let container_disabled_border = move || {
        matches!(
            kind,
            IconButtonKind::Outlined | IconButtonKind::OutlinedToggle
        )
    };

    let icon_text_on_surface_variant = move || match kind {
        IconButtonKind::Standard => true,
        IconButtonKind::StandardToggle => !toggled.get(),
        IconButtonKind::FilledTonalToggle => !toggled.get(),
        IconButtonKind::Outlined => true,
        IconButtonKind::OutlinedToggle => !toggled.get(),
        _ => false,
    };

    let icon_text_primary = move || match kind {
        IconButtonKind::StandardToggle => toggled.get(),
        IconButtonKind::FilledToggle => !toggled.get(),
        _ => false,
    };

    let icon_text_on_primary = move || match kind {
        IconButtonKind::Filled => true,
        IconButtonKind::FilledToggle => toggled.get(),
        _ => false,
    };

    let icon_text_on_secondary_container = move || match kind {
        IconButtonKind::FilledTonal => true,
        IconButtonKind::FilledTonalToggle => toggled.get(),
        _ => false,
    };

    view! {
        <button
            class="h-10 w-10 rounded-full relative"
            disabled=move || disabled.get()
            on:click = move |_| (on_click)()
        >
            <div
                id="container"
                class="absolute left-0 top-0 z-0 h-full w-full rounded-full border-outline"

                class=("bg-transparent", container_bg_transparent)
                class=("bg-primary", container_bg_primary)
                class=("bg-surface-variant", container_bg_surface_variant)
                class=("bg-secondary-container", container_bg_secondary_container)
                class=("bg-inverse-surface", move || matches!(kind, IconButtonKind::OutlinedToggle) && toggled.get())
                class=("border", container_border)
                class=("border-0", move || matches!(kind, IconButtonKind::OutlinedToggle) && toggled.get())

                class=("invisible", move || disabled.get())
            >
            </div>

            <div
                id="container-disabled"
                class="absolute left-0 top-0 z-0 h-full w-full rounded-full border-outline opacity-[.12]"

                class=("bg-transparent", container_disabled_bg_transparent)
                class=("bg-on-surface", container_disabled_bg_on_surface)
                class=("border", container_disabled_border)

                class=("invisible", move || !disabled.get())
            >

            </div>

            <div
                id="icon"
                class="absolute left-2 top-2 w-6 h-6 z-10"

                class=("text-on-surface-variant", icon_text_on_surface_variant)
                class=("text-primary", icon_text_primary)
                class=("text-on-primary", icon_text_on_primary)
                class=("text-on-secondary-container", icon_text_on_secondary_container)
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
