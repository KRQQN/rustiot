use leptos::prelude::*;

#[component]
pub fn SideMenu() -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);

    let menu_items = [
        ("Home", "🏠", "/"),
        ("Devices", "📱", "/devices"),
        ("Statistics", "📊", ""),
        ("Events", "📅", ""),
        ("Members", "👥", ""),
    ];

    view! {
        <aside
            class=move || {
                format!(
                    "sticky top-0 h-screen bg-slate-900 text-white \
                     transition-all duration-300 ease-in-out flex flex-col shrink-0 {}",
                    if collapsed.get() { "w-16" } else { "w-52" }
                )
            }
        >
            // Header
            <div class="flex items-center justify-between p-4 border-b border-slate-700 shrink-0">
                <Show when=move || !collapsed.get()>
                    <span class="font-bold text-md ml-4 tracking-wide">"Menu"</span>
                </Show>
                <button
                    class="p-2 rounded-lg hover:bg-slate-700 transition-colors"
                    on:click=move |_| set_collapsed.update(|c| *c = !*c)
                    aria-label="Toggle sidebar"
                >
                    <svg
                        class="w-5 h-5"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d=move || {
                                if collapsed.get() {
                                    "M4 6h16M4 12h16M4 18h16"
                                } else {
                                    "M6 18L18 6M6 6l12 12"
                                }
                            }
                        />
                    </svg>
                </button>
            </div>

            // Top menu items
            <nav class="flex-1 py-4 space-y-1 overflow-y-auto">
                {menu_items
                    .into_iter()
                    .map(|(label, icon, path)| {
                        view! {
                            <a
                                href=path
                                class="flex items-center gap-3 px-4 py-3 mx-2 rounded-lg
                                       hover:bg-slate-700 transition-colors"
                            >
                                <span class="text-xl w-6 text-center shrink-0">{icon}</span>
                                <Show when=move || !collapsed.get()>
                                    <span class="truncate text-md font-medium">{label}</span>
                                </Show>
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>

            // Settings at bottom
            <div class="border-t border-slate-700 p-2 shrink-0">
                <a
                    href="#"
                    class="flex items-center gap-3 px-4 py-3 rounded-lg
                           hover:bg-slate-700 transition-colors"
                >
                    <span class="text-xl w-6 text-center shrink-0">"⚙️"</span>
                    <Show when=move || !collapsed.get()>
                        <span class="truncate text-md font-medium">"Settings"</span>
                    </Show>
                </a>
            </div>
        </aside>
    }
}
