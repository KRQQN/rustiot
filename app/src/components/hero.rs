use crate::components::charts::BarChart;
use icons::{ChartColumn, Crosshair, Eye, ShieldCheck};
use leptos::logging::log;
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
pub fn Hero() -> impl IntoView {
    let data = RwSignal::new(DashboardData {
        active_devices: 1248,
        data_points: "2.4M".to_string(),
        online_percent: 90,
        growth_devices: 18.6,
        growth_points: 32.4,
    });

    view! {
                <section class="relative min-h-screen w-full overflow-hidden flex items-center justify-center mx-auto hero">

    <div
            class="absolute left-0 top-0 w-full h-full z-0"
            style="background-image: url('/iot-mesh-bg.jpg');
               background-size: cover;
               background-position: center center;
               filter: brightness(0.9) contrast(1.05) saturate(1.1);"
        ></div>

        <div class="absolute left-0 top-0 w-full h-full bg-gradient-to-r from-black/90 via-black/65 to-transparent z-10"></div>

                        <div class="relative z-20 w-full max-w-screen px-8 lg:px-[10rem] flex items-center">
                            <div class="w-full flex flex-col lg:flex-row justify-between items-center gap-12">

                                {/* LEFT SECTION */}
                                <div class="flex-1 flex-col justify-center p-8">
                                    <div class="flex items-center gap-4">
                                        <div class="w-12 h-12 bg-white rounded-2xl flex items-center justify-center text-black font-bold text-3xl">
                                            N
                                        </div>
                                        <div>
                                            <div class="text-4xl text-left font-semibold tracking-tighter text-white">NEXTIOT</div>
                                            <div class="text-slate-300 text-sm font-medium tracking-widest">INTELLIGENT IOT SOLUTIONS</div>
                                        </div>
                                    </div>
                                    <div class="space-y-2 mt-6">
                                        <h1 class="text-6xl lg:text-7xl font-bold text-white tracking-tighter leading-none text-left">
                                            MEASURE.
                                        </h1>
                                        <h1 class="text-6xl lg:text-7xl font-bold tracking-tighter leading-none text-center flex justify-center">
                                            {
                                                "VISUALISE.".chars().enumerate().map(|(i, c)| {
                                                    let is_blue = match i % 2 {
                                                        0 => true,
                                                        1 => false,
                                                        _ => true,
                                                    };
                                                    let color_class = if is_blue {
                                                        "text-cyan-500"
                                                    } else {
                                                        "text-white"
                                                    };
                                                    view! {
                                                        <span class=format!("{} inline-block hover:scale-110 transition-transform", color_class)>
                                                            {c.to_string()}
                                                        </span>
                                                    }
                                                }).collect_view()
                                            }
                                        </h1>
                                        <h1 class="text-6xl lg:text-7xl font-bold text-white tracking-tighter leading-none text-right">
                                            OPTIMISE.
                                        </h1>
                                    </div>
                                    <p class="text-xl text-left text-slate-200 mt-6 max-w-md">
                                    End-to-end IoT solutions that deliver precise measurement, real-time visualisation, and actionable optimisation to boost efficiency and reduce costs.
                                    </p>
                                    <div class="flex justify-between max-w-md text-base mt-8">
                                        <div class="flex flex-col w-16 items-center gap-3 text-white">
                                            <Crosshair class="size-12 text-cyan-400 drop-shadow-[0_0_8px_#22d3ee] drop-shadow-[0_0_20px_#22d3ee]"/>
                                            <span class="text-sm text-center">Accurate Measurement</span>
                                        </div>
                                        <div class="flex flex-col w-16 items-center gap-3 text-white">
                                            <Eye class="size-12 text-cyan-400 drop-shadow-[0_0_8px_#22d3ee] drop-shadow-[0_0_20px_#22d3ee]"/>
                                            <span class="text-sm text-center">Real-time Visualisation</span>
                                        </div>
                                        <div class="flex flex-col w-16 items-center gap-3 text-white">
                                            <ChartColumn class="size-12 text-cyan-400 drop-shadow-[0_0_8px_#22d3ee] drop-shadow-[0_0_20px_#22d3ee]"/>
                                            <span class="text-sm text-center">Actionable Insights</span>
                                        </div>
                                        <div class="flex flex-col w-16 items-center gap-3 text-white">
                                            <ShieldCheck class="size-12 text-cyan-400 drop-shadow-[0_0_8px_#22d3ee] drop-shadow-[0_0_20px_#22d3ee]"/>
                                            <span class="text-sm text-center">Reliable and Secure</span>
                                        </div>
                                    </div>
                                    <button
                                        class="px-8 py-5 mt-8 bg-cyan-500 hover:bg-cyan-300 text-black font-semibold text-xl rounded-3xl transition-all duration-300 shadow-2xl shadow-cyan-500/20 flex items-center gap-3"
                                        on:click=move |_| log!("Explore Solutions clicked")
                                    >
                                        Explore Our Solutions
                                    </button>
                                </div>

                                {/* RIGHT SECTION */}
                                <div class="perspective-wrapper flex-1 flex justify-center">
                                    <div class="dashboard-content">
                                        <Dashboard data=data />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>
                }
}

#[component]
pub fn Dashboard(data: RwSignal<DashboardData>) -> impl IntoView {
    view! {
            <div class="space-y-2 p-2">
                    <div class="flex justify-around text-sm mb-4 text-slate-400">
                        <span>DEVICE ACTIVITY</span>
                        <span>24 HOURS</span>
                    </div>
                    <div class="h-[13rem]">
                        <BarChart/>
                    </div>
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
#[component]
fn Metric<F1, F2>(label: &'static str, value: F1, growth: F2) -> impl IntoView
where
    F1: Fn() -> String + Send + 'static,
    F2: Fn() -> f32 + Send + 'static,
{
    view! {
        <div class="bg-[#1a2338] rounded-3xl p-1">
            <p class="text-slate-400 text-sm">{label}</p>
            <p class="text-xl font-semibold text-white mt-1">{move || value()}</p>
            <p class="text-emerald-400 text-sm font-medium mt-1">
                "↑ " {move || format!("{:.1}%", growth())}
            </p>
        </div>
    }
}
