use leptos::*;

/// Диалог
///
/// https://m3.material.io/components/dialogs/overview
#[component]
pub fn Dialog<F1, IV1>(
    /// Сигнал управления видимостью диалога
    #[prop(into)]
    visible: Signal<bool>,

    on_close: impl Fn() -> () + Copy + 'static,

    /// Заголовок диалога
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Headline }
    /// ```
    headline: F1,
) -> impl IntoView
where
    F1: Fn() -> IV1 + 'static,
    IV1: IntoView,
{
    // let on_close_clone = on_close.clone();
    view! {
        <Show when=move || visible.get()>
            <div
                class="relative z-10"
                aria-labelledby="modal-title"
                role="dialog"
                aria-modal="true"
            >
                // <!--
                // Background backdrop, show/hide based on modal state.

                // Entering: "ease-out duration-300"
                // From: "opacity-0"
                // To: "opacity-100"
                // Leaving: "ease-in duration-200"
                // From: "opacity-100"
                // To: "opacity-0"
                // -->
                <div
                    class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
                    class=("opacity-0", move || !visible.get())
                    class=("opacity-100", move || visible.get())
                ></div>

                <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                    <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        // <!--
                        // Modal panel, show/hide based on modal state.

                        // Entering: "ease-out duration-300"
                        // From: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        // To: "opacity-100 translate-y-0 sm:scale-100"
                        // Leaving: "ease-in duration-200"
                        // From: "opacity-100 translate-y-0 sm:scale-100"
                        // To: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        // -->
                        <div class="relative transform overflow-hidden rounded-3xl bg-surface-container-high text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm p-6">
                            <div>
                                <div class="text-center">
                                    <h3
                                        class="text-base font-semibold leading-6 text-on-surface"
                                        id="modal-title"
                                    >
                                        {headline()}
                                    </h3>
                                    <div class="mt-2">
                                        <p class="text-sm text-on-surface-variant">
                                            Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur amet labore.
                                        </p>
                                    </div>
                                </div>
                            </div>
                            <div class="mt-5 sm:mt-6">
                                <button
                                    type="button"
                                    class="inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"

                                    on:click=move |_| on_close()
                                >
                                    Go back to dashboard
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
