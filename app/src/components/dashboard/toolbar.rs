use crate::pages::dashboard::PanelType;
use leptos::prelude::*;

#[component]
pub fn PanelToolbar(on_add: Callback<PanelType>) -> impl IntoView {
    let (collapsed, set_collapsed) = signal(false);
    let (add_open, set_add_open) = signal(true); // open by default so it feels useful

    view! {
        <aside
            class=move || {
                format!(
                    "fixed top-0 right-0 h-screen z-50 flex flex-col \
                     bg-[#0f0f0f] border-l border-neutral-800 \
                     transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] {}",
                    if collapsed.get() { "w-[52px]" } else { "w-[260px]" }
                )
            }
        >
            // HEADER
            <div class="flex items-center h-14 px-3 border-b border-neutral-800 shrink-0">
                <Show when=move || !collapsed.get()>
                    <span class="text-[13px] font-semibold tracking-wide text-neutral-200">
                        "Toolbar"
                    </span>
                </Show>

                <div class="flex-1" />

                <button
                    class="w-8 h-8 flex items-center justify-center rounded-md
                           text-neutral-500 hover:text-neutral-200 hover:bg-neutral-800
                           transition-colors"
                    on:click=move |_| {
                        set_collapsed.update(|c| {
                            *c = !*c;
                            if *c {
                                set_add_open.set(false);
                            }
                        });
                    }
                    title=move || if collapsed.get() { "Expand" } else { "Collapse" }
                >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d=move || {
                                if collapsed.get() {
                                    "M9 5l7 7-7 7"
                                } else {
                                    "M15 19l-7-7 7-7"
                                }
                            }
                        />
                    </svg>
                </button>
            </div>

            // CONTENT
            <div class="flex-1 overflow-y-auto py-3">
                <div class="px-2">
                    <button
                        class=move || {
                            let base = "w-full flex items-center gap-3 rounded-lg transition-all duration-150 ";
                            if collapsed.get() {
                                format!("{base} justify-center h-10 text-neutral-400 hover:text-white hover:bg-neutral-800")
                            } else {
                                format!("{base} h-10 px-3 bg-white text-black hover:bg-neutral-100 font-medium text-[13px]")
                            }
                        }
                        on:click=move |_| {
                            if collapsed.get() {
                                set_collapsed.set(false);
                                set_add_open.set(true);
                            } else {
                                set_add_open.update(|v| *v = !*v);
                            }
                        }
                    >
                        <svg class="w-4 h-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                        </svg>

                        <Show when=move || !collapsed.get()>
                            <span class="flex-1 text-left">"Add panel"</span>
                            <svg
                                class=move || {
                                    format!(
                                        "w-3.5 h-3.5 text-neutral-500 transition-transform duration-200 {}",
                                        if add_open.get() { "rotate-180" } else { "" }
                                    )
                                }
                                fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                            </svg>
                        </Show>
                    </button>

                    <Show when=move || add_open.get() && !collapsed.get()>
                        <div class="mt-1 space-y-0.5">
                            // BAR CHART
                            <button
                                class="w-full flex items-center gap-3 h-9 px-3 rounded-md
                                       text-[13px] text-neutral-400 hover:text-white hover:bg-neutral-800/80
                                       transition-colors group"
                                on:click=move |_| {
                                    on_add.run(PanelType::BarChart);
                                }
                            >
                                <div class="w-5 h-5 rounded bg-blue-500/10 flex items-center justify-center text-blue-400 group-hover:bg-blue-500/20">
                                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M4 20h3V10H4v10zm6 0h3V4h-3v16zm6 0h3v-8h-3v8z"/>
                                    </svg>
                                </div>
                                "Bar chart"
                            </button>

                            // LINE CHART
                            <button
                                class="w-full flex items-center gap-3 h-9 px-3 rounded-md
                                       text-[13px] text-neutral-400 hover:text-white hover:bg-neutral-800/80
                                       transition-colors group"
                                on:click=move |_| {
                                    on_add.run(PanelType::LineChart);
                                }
                            >
                                <div class="w-5 h-5 rounded bg-emerald-500/10 flex items-center justify-center text-emerald-400 group-hover:bg-emerald-500/20">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.5 16.5l4.5-4.5 3.5 3.5 6.5-7"/>
                                    </svg>
                                </div>
                                "Line chart"
                            </button>

                            // CALENDAR
                            <button
                                class="w-full flex items-center gap-3 h-9 px-3 rounded-md
                                       text-[13px] text-neutral-400 hover:text-white hover:bg-neutral-800/80
                                       transition-colors group"
                                on:click=move |_| {
                                    on_add.run(PanelType::CalendarHeatmap);
                                }
                            >
                                <div class="w-5 h-5 rounded bg-amber-500/10 flex items-center justify-center text-amber-400 group-hover:bg-amber-500/20">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                              d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                    </svg>
                                </div>
                                "Calendar heatmap"
                            </button>
                        </div>
                    </Show>
                </div>

                // TODO: Future tools

            </div>

            // FOOTER
            <div class="border-t border-neutral-800 p-2 shrink-0">
                <button
                    class="w-full h-9 flex items-center gap-3 px-3 rounded-md
                           text-neutral-500 hover:text-neutral-300 hover:bg-neutral-800/50
                           transition-colors text-[13px]"
                >
                    <svg class="w-4 h-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.8">
                        <path stroke-linecap="round" stroke-linejoin="round"
                              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                    <Show when=move || !collapsed.get()>
                        <span>"Settings"</span>
                    </Show>
                </button>
            </div>
        </aside>
    }
}
