use crate::components::js_charts::{ECalendarHeatmap, ETreeChart};

use super::dashboard::DashboardData;
use leptos::prelude::*;
use serde_json::json;

#[component]
pub fn Platform() -> impl IntoView {
    let tree = Signal::derive(|| {
        json!({
            "name": "Dashboard",
            "children": [
                {
                    "name": "Production",
                    "children": [
                        {
                            "name": "Sensors",
                            "children": [
                                { "name": "Power" },
                                { "name": "Usage" },
                                { "name": "Status" }
                            ]
                        },
                        {
                            "name": "Alarms",
                            "children": [
                                { "name": "Leak" },
                                { "name": "Usage" },
                                { "name": "AI" }
                            ]
                        },
                    ]
                },
                {
                    "name": "Garage",
                    "children": [
                        { "name": "GPS" },
                        { "name": "Tanks" },
                        { "name": "Alarms" }
                    ]
                },
            ]
        })
    });

    view! {
        <section class="relative overflow-hidden bg-[#0B1220] border-y border-[#18263B]">

            <div class="absolute -left-40 top-1/2 -translate-y-1/2 w-[32rem] h-[32rem] rounded-full bg-cyan-500/5 blur-[180px]" />

            <div class="absolute -right-56 bottom-0 w-[36rem] h-[36rem] rounded-full bg-cyan-400/5 blur-[220px]" />

            <div class="max-w-7xl mx-auto px-6 sm:px-8 lg:px-12 py-24 lg:py-36">

                <div class="grid grid-cols-1 lg:grid-cols-2 lg:gap-24 items-center">

                    <div class="order-2 lg:order-1">

                        <span class="uppercase tracking-[0.35em] text-cyan-400 text-sm font-semibold">
                            Platform
                        </span>

                        <h2 class="mt-5 text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight leading-[0.95] text-white">
                            One Platform.
                            <br/>
                            Every Connected Asset.
                        </h2>

                        <p class="mt-8 text-slate-300 text-lg leading-8 max-w-xl">

                            One platform connecting every device,
                            sensor and location.

                            Monitor assets in real time, automate critical
                            responses and stay informed with intelligent alerts.

                            From smart homes to industrial facilities,
                            everything is managed securely from one dashboard.
                        </p>

                    <div class="hidden lg:block">
                        <div class="mt-12 grid sm:grid-cols-2 lg:gap-5">

                            <div class="rounded-2xl border border-[#223149] bg-[#101827]/70 p-5">
                                <h3 class="text-white font-semibold">
                                    Live Monitoring
                                </h3>

                                <p class="mt-2 text-sm leading-6 text-slate-400">
                                    View every connected sensor in real time.
                                </p>
                            </div>

                            <div class="rounded-2xl border border-[#223149] bg-[#101827]/70 p-5">
                                <h3 class="text-white font-semibold">
                                    Smart Alerts
                                </h3>

                                <p class="mt-2 text-sm leading-6 text-slate-400">
                                    Receive notifications before failures occur.
                                </p>
                            </div>

                            <div class="rounded-2xl border border-[#223149] bg-[#101827]/70 p-5">
                                <h3 class="text-white font-semibold">
                                    Analytics
                                </h3>

                                <p class="mt-2 text-sm leading-6 text-slate-400">
                                    Discover trends from historical telemetry.
                                </p>
                            </div>

                            <div class="rounded-2xl border border-[#223149] bg-[#101827]/70 p-5">
                                <h3 class="text-white font-semibold">
                                    Secure Cloud
                                </h3>

                                <p class="mt-2 text-sm leading-6 text-slate-400">
                                    Enterprise-grade infrastructure and encryption.
                                </p>
                            </div>

                        </div>

                        <button class="mt-12 px-8 py-5 rounded-3xl bg-cyan-500 hover:bg-cyan-300 transition-all duration-300 text-black font-semibold shadow-xl shadow-cyan-500/20">
                            Explore Platform
                        </button>
                    </div>

                    </div>

                    <div class="order-2 lg:order-2 flex justify-center">

                    <div class="w-full h-[700px] max-w-xl xl:max-w-2xl">
                        <ETreeChart data=tree/>
                    </div>
                    </div>

                </div>

            </div>

        </section>
    }
}
