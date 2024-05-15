use leptos::*;

/// https://tailwindui.com/components/application-ui/forms/toggles
#[component]
pub fn SimpleToggle(
    /// Состояние переключателя
    #[prop(into)]
    status: Signal<bool>,

    /// Событие переключения
    clicked: WriteSignal<()>,
) -> impl IntoView {
    view! {
        <button type="button" class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2" role="switch" aria-checked="false"
        class = ("bg-gray-200", move || !status.get())
        class = ("bg-indigo-600", move || status.get())
        on:click= move |_| clicked.set(())
        >
            <span class="sr-only">Use setting</span>
            <span aria-hidden="true" class="translate-x-0 pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
            class = ("translate-x-0", move || !status.get())
            class = ("translate-x-5", move || status.get())
            ></span>
        </button>
    }
}
