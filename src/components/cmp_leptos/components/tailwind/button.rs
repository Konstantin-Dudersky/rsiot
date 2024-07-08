use leptos::*;

#[component]
pub fn Button<FClicked>(
    /// Текст кнопки
    #[prop(default = "Button text")]
    text: &'static str,

    /// Событие нажатия
    clicked: FClicked,
) -> impl IntoView
where
    FClicked: Fn() + 'static,
{
    view! {
        <button type="button" class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        on:click= move |_| (clicked)()
        >{ text }</button>

    }
}
