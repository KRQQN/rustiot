use leptos::prelude::*;

use crate::components::charts::{BarChart, LineChart, NetworkChart, TrackingChart};

#[component]
pub fn Solutions() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden bg-[#0B1220] border-y border-[#18263B]">

            <div class="absolute right-0 top-20 w-[34rem] h-[34rem] rounded-full bg-cyan-500/5 blur-[200px]" />

            <div class="max-w-7xl mx-auto px-6 sm:px-8 lg:px-12 py-24 lg:py-36">

                <div class="max-w-3xl">

                    <span class="uppercase tracking-[0.35em] text-cyan-400 text-sm font-semibold">
                        Solutions
                    </span>

                    <h2 class="mt-5 text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight leading-[0.95] text-white">
                        Intelligent Solutions
                        <br/>
                        For Every Industry.
                    </h2>

                    <p class="mt-8 text-lg leading-8 text-slate-300">
                        From monitoring environmental conditions to automating
                        industrial infrastructure, NEXTIOT delivers intelligent
                        systems that transform operational data into actionable
                        insights.
                    </p>

                </div>

                <div class="mt-20 space-y-10">

                    // <SolutionCard
                    //     title="Environmental Monitoring"
                    //     description="Monitor temperature, humidity and environmental conditions with intelligent alerts and historical analytics."
                    //     reverse=false
                    // >
                    //     <LineChart/>
                    // </SolutionCard>

                    <SolutionCard
                        title="Energy Intelligence"
                        description="Track power consumption, identify inefficiencies and optimise energy usage across your facilities."
                        reverse=true
                    >
                        <BarChart/>
                    </SolutionCard>

                    <SolutionCard
                        title="Industrial Automation"
                        description="Connect PLCs, machinery and smart devices into one secure industrial platform."
                        reverse=false
                    >
                        <NetworkChart/>
                    </SolutionCard>

                    <SolutionCard
                        title="Asset Tracking"
                        description="Monitor equipment locations, movement history and asset utilisation in real time."
                        reverse=true
                    >
                        <TrackingChart/>
                    </SolutionCard>

                </div>

            </div>

        </section>
    }
}

#[component]
fn SolutionCard(
    title: &'static str,
    description: &'static str,
    reverse: bool,
    children: Children,
) -> impl IntoView {
    let grid = if reverse {
        "lg:grid-cols-2 lg:[&>*:first-child]:order-2"
    } else {
        "lg:grid-cols-2"
    };

    view! {

        <div class=format!(
            "grid grid-cols-1 {} gap-10 items-center rounded-[32px] border border-[#223149] bg-[#101827] p-8 lg:p-10",
            grid
        )>

            <div class="relative h-72 lg:h-80 rounded-[24px] overflow-hidden border border-[#1F314A] bg-[#0D1524]">

                <div class="absolute inset-0 bg-gradient-to-br from-cyan-500/10 via-transparent to-transparent"/>

                <div class="absolute inset-4">
                    {children()}
                </div>

            </div>

            <div class="flex flex-col">

                <span class="uppercase tracking-[0.25em] text-cyan-400 text-xs">
                    Solution
                </span>

                <h3 class="mt-4 text-3xl lg:text-4xl font-semibold text-white">
                    {title}
                </h3>

                <p class="mt-6 text-lg leading-8 text-slate-400">
                    {description}
                </p>

                <div class="mt-10 flex flex-wrap gap-3">

                    <span class="px-4 py-2 rounded-full border border-cyan-500/20 bg-cyan-500/10 text-cyan-300 text-sm">
                        Real-Time
                    </span>

                    <span class="px-4 py-2 rounded-full border border-cyan-500/20 bg-cyan-500/10 text-cyan-300 text-sm">
                        Secure
                    </span>

                    <span class="px-4 py-2 rounded-full border border-cyan-500/20 bg-cyan-500/10 text-cyan-300 text-sm">
                        Scalable
                    </span>

                </div>

                <button class="mt-10 w-fit rounded-2xl bg-cyan-500 px-7 py-4 font-semibold text-black shadow-lg shadow-cyan-500/20 transition-all duration-300 hover:bg-cyan-300">
                    Learn More
                </button>

            </div>

        </div>

    }
}
