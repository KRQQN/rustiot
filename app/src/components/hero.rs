use crate::components::dashboard_example::{DashboardData, DashboardExample};
use icons::{ChartColumn, Crosshair, Eye, ShieldCheck};
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    // todo -> move this dummy to dashboard
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

            <div class="absolute left-0 top-0 w-full h-full bg-gradient-to-r from-black via-black/55 to-transparent z-10"></div>

                <div class="relative z-20 w-full max-w-[65%] lg:px-[10rem] flex items-center">
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
                                <DashboardExample data=data />
                            </div>
                        </div>
                    </div>
                </div>
        </section>
    }
}
