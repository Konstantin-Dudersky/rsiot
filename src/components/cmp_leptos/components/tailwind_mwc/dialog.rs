use leptos::*;

/// Диалог
///
/// https://m3.material.io/components/dialogs/overview
#[component]
pub fn Dialog<FHeadline, IvHeadline, FContent, IvContent, FActions, IvActions>(
    /// Сигнал управления видимостью диалога
    #[prop(into)]
    visible: Signal<bool>,

    /// Заголовок диалога
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Headline }
    /// ```
    headline: FHeadline,

    /// Содержимое диалога
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Content }
    /// ```
    content: FContent,

    /// Содержимое области действий
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Actions }
    /// ```
    actions: FActions,
) -> impl IntoView
where
    FHeadline: Fn() -> IvHeadline + 'static,
    IvHeadline: IntoView,

    FContent: Fn() -> IvContent + 'static,
    IvContent: IntoView,

    FActions: Fn() -> IvActions + 'static,
    IvActions: IntoView,
{
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
                    <div class="flex min-h-full justify-center p-4 text-center items-center">
                        // <!--
                        // Modal panel, show/hide based on modal state.

                        // Entering: "ease-out duration-300"
                        // From: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        // To: "opacity-100 translate-y-0 sm:scale-100"
                        // Leaving: "ease-in duration-200"
                        // From: "opacity-100 translate-y-0 sm:scale-100"
                        // To: "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                        // -->

                        <div class="relative transform overflow-hidden rounded-3xl bg-surface-container-high text-left shadow-xl transition-all sm:my-8 ">
                            <div class="flex flex-col p-6">
                                <div class="text-center mb-4">
                                    <h3
                                        class="text-base font-semibold leading-6 text-on-surface"
                                        id="modal-title"
                                    >
                                        {headline()}
                                    </h3>
                                </div>
                                <div class="mb-6">{content()}</div>
                                <div class="flex flex-row justify-end">{actions()}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
