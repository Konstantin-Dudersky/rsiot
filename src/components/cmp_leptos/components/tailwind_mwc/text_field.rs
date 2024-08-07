use leptos::*;
use web_sys::KeyboardEvent;

/// Тип визуального отображения
pub enum TextFieldKind {
    /// https://m3.material.io/components/text-fields/specs#68b00bd6-ab40-4b4f-93d9-ed1fbbc5d06e
    Outline,
}

/// Тип HTML-элемента <input/>
pub enum InputType {
    /// type="text"
    Text,
    /// type="number"
    Number,
}

impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Text => "text",
            InputType::Number => "number",
        }
        .to_string()
    }
}

#[component]
pub fn TextField(
    /// Значение
    #[prop(into)]
    value: Signal<String>,

    /// Событие нажатия кнопки "Enter"
    on_keyup_enter: impl Fn(&str) + 'static,

    /// Вид поля ввода
    #[prop(default=TextFieldKind::Outline)]
    _kind: TextFieldKind,

    /// Подпись сверху элемента
    #[prop(default = "")]
    label_text: &'static str,

    /// Тип HTML-элемента <input/>
    #[prop(default = InputType::Text)]
    input_type: InputType,
) -> impl IntoView {
    let (input_text, input_text_set) = create_signal(String::from(""));

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
                sm:text-sm sm:leading-6 h-14" placeholder="Jane Smith"

                type=input_type.to_string()

                value = move || value.get()

                on:keyup=move |event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        on_keyup_enter(&input_text.get())
                    }
                }

                on:input = move |ev| {
                    let value = event_target_value(&ev);
                    input_text_set.set(value);
                }
            />
        </div>
    }
}

// ring-1 ring-inset ring-gray-300
// focus:ring-2 focus:ring-inset focus:ring-indigo-600
