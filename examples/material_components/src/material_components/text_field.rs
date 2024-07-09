use leptos::*;

use rsiot::components::cmp_leptos::components::tailwind_mwc::{InputType, TextField};
use tracing::info;

#[component]
pub fn TextFieldView() -> impl IntoView {
    let (test_str, test_str_set) = create_signal(String::from(""));

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
            input_type=InputType::Number
        />

    }
}
