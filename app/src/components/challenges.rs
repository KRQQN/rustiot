use icons::{DollarSign, Eye, HatGlasses, Wifi};
use leptos::prelude::*;

#[component]
pub fn Problems() -> impl IntoView {
    view! {
        <section class="bg-[#07101C] border-y border-[#18263B] overflow-hidden">
            <div class="max-w-7xl mx-auto px-4 py-20 lg:py-28">
                <div class="grid grid-cols-1 lg:grid-cols-[0.9fr_1.5fr] gap-12 lg:gap-16 items-center">
                    <div>
                        <span class="uppercase tracking-[0.35em] text-cyan-400 text-sm font-semibold">
                            "Challenges"
                        </span>
                        <h2 class="mt-6 text-4xl lg:text-5xl font-bold tracking-tight leading-tight text-white">
                            "Every Inefficiency"
                            <br />
                            "Starts With"
                            <br />
                            "Missing Data."
                        </h2>
                        <p class="mt-8 text-lg text-slate-300 max-w-md leading-relaxed">
                            "Modern operations demand continuous visibility,
                            accurate measurements and intelligent automation.
                            Without them, costs increase while productivity
                            suffers."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
                        // Card 1
                        <div class="group rounded-3xl border border-[#223149] bg-[#101827] p-6 transition-colors hover:border-cyan-500/30 hover:bg-[#121f30]">
                            <div class="flex items-start gap-4">
                                <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-cyan-500/20 bg-cyan-500/10 text-cyan-400 transition-colors group-hover:border-cyan-500/40 group-hover:bg-cyan-500/15">
                                    <HatGlasses />
                                </div>
                                <div>
                                    <h3 class="text-lg font-semibold text-white leading-tight">
                                        "Manual Monitoring"
                                    </h3>
                                    <p class="mt-2 text-slate-400 leading-relaxed text-[15px]">
                                        "Collecting measurements manually wastes valuable time and introduces unnecessary human error."
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Card 2
                        <div class="group rounded-3xl border border-[#223149] bg-[#101827] p-6 transition-colors hover:border-cyan-500/30 hover:bg-[#121f30]">
                            <div class="flex items-start gap-4">
                                <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-cyan-500/20 bg-cyan-500/10 text-cyan-400 transition-colors group-hover:border-cyan-500/40 group-hover:bg-cyan-500/15">
                                    <Wifi />
                                </div>
                                <div>
                                    <h3 class="text-lg font-semibold text-white leading-tight">
                                        "Disconnected Systems"
                                    </h3>
                                    <p class="mt-2 text-slate-400 leading-relaxed text-[15px]">
                                        "Critical devices operate in isolation, making data difficult to access and manage."
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Card 3
                        <div class="group rounded-3xl border border-[#223149] bg-[#101827] p-6 transition-colors hover:border-cyan-500/30 hover:bg-[#121f30]">
                            <div class="flex items-start gap-4">
                                <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-cyan-500/20 bg-cyan-500/10 text-cyan-400 transition-colors group-hover:border-cyan-500/40 group-hover:bg-cyan-500/15">
                                    <Eye />
                                </div>
                                <div>
                                    <h3 class="text-lg font-semibold text-white leading-tight">
                                        "No Real-Time Visibility"
                                    </h3>
                                    <p class="mt-2 text-slate-400 leading-relaxed text-[15px]">
                                        "Operational issues often remain hidden until downtime has already impacted productivity."
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Card 4
                        <div class="group rounded-3xl border border-[#223149] bg-[#101827] p-6 transition-colors hover:border-cyan-500/30 hover:bg-[#121f30]">
                            <div class="flex items-start gap-4">
                                <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-cyan-500/20 bg-cyan-500/10 text-cyan-400 transition-colors group-hover:border-cyan-500/40 group-hover:bg-cyan-500/15">
                                    <DollarSign />
                                </div>
                                <div>
                                    <h3 class="text-lg font-semibold text-white leading-tight">
                                        "Rising Operating Costs"
                                    </h3>
                                    <p class="mt-2 text-slate-400 leading-relaxed text-[15px]">
                                        "Inefficient maintenance and poor resource utilisation increase long-term expenses."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
