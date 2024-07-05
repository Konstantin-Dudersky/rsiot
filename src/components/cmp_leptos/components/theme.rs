use leptos::*;

const BASE_STYLE: &str = "h-full bg-background text-on-background";

/// Переключение темы.
///
/// У элемента `body` необходимо задать `id=body`
#[component]
pub fn Theme(
    /// Строковое именование темы
    ///
    /// Для Material Theme допустимые значения:
    /// - dark-high-contrast
    /// - dark-medium-contrast
    /// - dark
    /// - light-high-contrast
    /// - light-medium-contrast
    /// - light
    #[prop(default = MaybeSignal::Static("light".into()))]
    theme: MaybeSignal<String>,
) -> impl IntoView {
    create_effect(move |_| {
        let theme = theme.get();
        let el = document().get_element_by_id("body").unwrap();
        let class_name = format!("{} {}", BASE_STYLE, theme);
        el.set_class_name(&class_name)
    });

    view! {}
}
