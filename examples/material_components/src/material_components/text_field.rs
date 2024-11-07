use leptos::prelude::*;

use rsiot::components::cmp_leptos::components::tailwind_mwc::{InputHtmlType, TextField};
use tracing::info;

#[component]
pub fn TextFieldView() -> impl IntoView {
    let (test_str, test_str_set) = signal(String::from(""));

    view! {
        <div class="h-14">
        </div>

        <TextField
            label_text = "Заголовок"
            value=test_str
            on_keyup_enter=move |new_value| {
                info!("New value: {}", new_value);
                test_str_set.set(new_value.to_string())
            }
            input_html_type=InputHtmlType::Number
        />

    }
}
