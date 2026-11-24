use leptos::prelude::*;

#[derive(Clone)]
pub struct DashboardData {
    pub active_devices: i32,
    pub data_points: String,
    pub online_percent: u8,
    pub growth_devices: f32,
    pub growth_points: f32,
}

#[component]
pub fn DashboardExample(data: RwSignal<DashboardData>) -> impl IntoView {
    view! {
            <div class="space-y-1 p-2">
            <div class="grid grid-cols-[1fr_1fr_0.9fr] gap-3">
        // Trends
        <div class="bg-[#111a2d] rounded-2xl px-4 py-3 border border-slate-800">
            <div class="flex items-center justify-between mb-3">
                <span class="text-[11px] tracking-[0.18em] text-slate-500">
                    TRENDS
                </span>
                <span class="text-xs font-medium text-emerald-400">
                    +14%
                </span>
            </div>
            <div class="h-20 flex items-end gap-1">
                {(0..8).map(|i| {
                    let height = 40 + (i % 3) * 20;
                    view! {
                        <div
                            class="flex-1 rounded-sm bg-cyan-300/90 hover:bg-cyan-200 transition-all"
                            style=move || format!("height: {}%", height)
                        />
                    }
                }).collect_view()}
            </div>
        </div>
        // Device Status
        <div class="bg-[#111a2d] rounded-2xl px-4 py-3 border border-slate-800">
            <p class="text-[11px] tracking-[0.18em] text-slate-500 text-center mb-3">
                DEVICE STATUS
            </p>
            <div class="relative flex justify-center items-center h-[120px]">
                <svg viewBox="0 0 42 42" class="absolute w-28 h-28 -rotate-90">
                    <circle
                        cx="21"
                        cy="21"
                        r="18"
                        fill="none"
                        stroke="#1e2937"
                        stroke-width="4"
                    />
                    <circle
                        cx="21"
                        cy="21"
                        r="18"
                        fill="none"
                        stroke="#67e8f9"
                        stroke-width="4"
                        stroke-dasharray="113"
                        stroke-dashoffset=move || {
                            let percent = data.get().online_percent as f32;
                            let circumference = 113.0;
                            format!(
                                "{:.2}",
                                circumference - (circumference * percent / 100.0)
                            )
                        }
                        stroke-linecap="round"
                    />
                </svg>
                <div class="z-10 flex flex-col items-center">
                    <span class="text-2xl font-semibold text-white leading-none">
                        {move || data.get().online_percent}%
                    </span>
                    <span class="text-[10px] tracking-[0.25em] text-emerald-400 mt-2">
                        ONLINE
                    </span>
                </div>
            </div>
        </div>
        // Stats
        <div class="flex flex-col gap-3">
            <div class="bg-[#111a2d] rounded-2xl px-4 py-3 border border-slate-800 flex flex-col justify-between flex-1">
                <span class="text-[11px] tracking-[0.12em] text-slate-500">
                    ACTIVE DEVICES
                </span>
                <div class="flex items-end justify-between mt-2">
                    <span class="text-3xl font-semibold text-white">
                        {move || data.get().active_devices}
                    </span>
                    <span class="text-sm font-medium text-emerald-400">
                        +{move || data.get().growth_devices}%
                    </span>
                </div>
            </div>
            <div class="bg-[#111a2d] rounded-2xl px-4 py-3 border border-slate-800 flex flex-col justify-between flex-1">
                <span class="text-[11px] tracking-[0.12em] text-slate-500">
                    DATA POINTS
                </span>
                <div class="flex items-end justify-between mt-2">
                    <span class="text-3xl font-semibold text-white">
                        {move || data.get().data_points.clone()}
                    </span>
                    <span class="text-sm font-medium text-emerald-400">
                        +{move || data.get().growth_points}%
                    </span>
                </div>
            </div>
        </div>
        </div>
    </div>
        }
}
