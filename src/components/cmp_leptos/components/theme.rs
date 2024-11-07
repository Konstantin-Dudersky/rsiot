use leptos::prelude::*;

const CLASS_NAMES: &str = "h-full bg-background text-on-background";
const STYLE: &str = "color-scheme: $color_scheme";

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
    #[prop(default = Signal::derive(|| "light".into()))]
    theme: Signal<String>,

    /// Свойство CSS color-scheme
    html_color_scheme: Signal<String>,
) -> impl IntoView {
    Effect::new(move |_| {
        let body_element = document().get_element_by_id("body").unwrap();

        let theme = theme.get();
        let class_name = format!("{} {}", CLASS_NAMES, theme);
        body_element.set_class_name(&class_name);

        let style = STYLE.replace("$color_scheme", &html_color_scheme.get());
        body_element.set_attribute("style", &style).unwrap();
    });
}
