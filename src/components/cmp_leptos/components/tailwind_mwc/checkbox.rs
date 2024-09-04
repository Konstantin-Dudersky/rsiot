use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <input type="checkbox"
        on:change = move |_| on_change()
        checked = move || checked.get()
        />

    }
}
