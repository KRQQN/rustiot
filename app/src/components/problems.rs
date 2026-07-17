use leptos::prelude::*;

#[component]
pub fn Problems() -> impl IntoView {
    let items = [
        (
            "Manual Monitoring",
            "Collecting measurements manually wastes valuable time and introduces unnecessary human error.",
        ),
        (
            "Disconnected Systems",
            "Critical devices operate in isolation, making data difficult to access and manage.",
        ),
        (
            "No Real-Time Visibility",
            "Operational issues often remain hidden until downtime has already impacted productivity.",
        ),
        (
            "Rising Operating Costs",
            "Inefficient maintenance and poor resource utilisation increase long-term expenses.",
        ),
    ];

    view! {
        <section class="bg-[#07101C] border-y border-[#18263B] overflow-hidden">
            <div class="max-w-7xl mx-auto px-4 py-20 lg:py-28">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
                    <div>
                        <span class="uppercase tracking-[0.35em] text-cyan-400 text-sm font-semibold">
                            Challenges
                        </span>
                        <h2 class="mt-6 text-4xl lg:text-5xl font-bold tracking-tight leading-tight text-white">
                            Every Inefficiency
                            <br/>
                            Starts With
                            <br/>
                            Missing Data.
                        </h2>
                        <p class="mt-8 text-lg text-slate-300 max-w-md">
                            Modern operations demand continuous visibility,
                            accurate measurements and intelligent automation.
                            Without them, costs increase while productivity
                            suffers.
                        </p>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
                        {
                            items.into_iter().map(|(title,body)| view! {
                                <div class="rounded-3xl border border-[#223149] bg-[#101827] p-8">
                                    <div class="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center">
                                        <div class="w-2.5 h-2.5 rounded-full bg-cyan-400"/>
                                    </div>
                                    <h3 class="mt-8 text-2xl font-semibold text-white">
                                        {title}
                                    </h3>
                                    <p class="mt-5 text-slate-400 leading-relaxed">
                                        {body}
                                    </p>
                                </div>
                            }).collect_view()
                        }
                    </div>
                </div>
            </div>
        </section>
    }
}
