use std::fmt::Display;

use leptos::prelude::*;
use web_sys::KeyboardEvent;

/// Тип визуального отображения
pub enum TextFieldKind {
    /// <https://m3.material.io/components/text-fields/specs#68b00bd6-ab40-4b4f-93d9-ed1fbbc5d06e>
    Outline,
}

/// Тип HTML-элемента <input/>
pub enum InputHtmlType {
    /// type="checkbox"
    Checkbox,

    /// type="color"
    Color,

    /// type="date"
    Date,

    /// type="datetime"
    Datetime,

    /// type="number"
    Number,

    /// type="text"
    Text,

    /// type="time"
    Time,
}

impl Display for InputHtmlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            InputHtmlType::Checkbox => "checkbox",
            InputHtmlType::Color => "color",
            InputHtmlType::Date => "date",
            InputHtmlType::Datetime => "datetime-local",
            InputHtmlType::Number => "number",
            InputHtmlType::Text => "text",
            InputHtmlType::Time => "time",
        };
        write!(f, "{}", txt)
    }
}

#[component]
pub fn TextField(
    /// Значение
    #[prop(into)]
    value: Signal<String>,

    /// Событие нажатия кнопки "Enter"
    on_keyup_enter: impl Fn(&str) + 'static + Copy,

    /// Вид поля ввода
    #[prop(default=TextFieldKind::Outline)]
    _kind: TextFieldKind,

    /// Подпись сверху элемента
    #[prop(default = "")]
    label_text: &'static str,

    /// Тип HTML-элемента <input/>
    #[prop(default = InputHtmlType::Text)]
    input_html_type: InputHtmlType,

    /// HTML placeholder
    #[prop(default = "Default")]
    placeholder: &'static str,
) -> impl IntoView {
    let (input_text, input_text_set) = signal(String::from(""));

    view! {
        <div class="relative">
            <label
                for="name"
                class="absolute -top-2 left-2 inline-block bg-surface px-1 text-xs font-medium text-on-surface-variant"
            >
                { label_text }
            </label>

            <input
                name="name"
                id="name"
                autocomplete="off"
                class="bg-surface block w-full rounded-md border-1 border-outline py-1.5
                text-on-surface-variant
                shadow-sm placeholder:text-gray-400
                focus:border-primary focus:border-2
                ring-0
                sm:text-sm sm:leading-6 h-14"

                placeholder=placeholder

                type=input_html_type.to_string()

                value = move || value.get()

                on:keyup=move |event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        on_keyup_enter(&input_text.get())
                    }
                }

                on:change=move |_| {
                    on_keyup_enter(&input_text.get())
                }

                on:input = move |ev| {
                    let value = event_target_value(&ev);
                    input_text_set.set(value);
                }
            />
        </div>
    }
}
