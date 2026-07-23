use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn BarChartMock() -> impl IntoView {
    let today = Local::now().date_naive();

    let day_labels: Vec<String> = (0..14)
        .rev()
        .map(|i| {
            let date = today - chrono::Duration::days(i);
            date.format("%m/%d").to_string()
        })
        .collect();

    let max_value = 1400;

    let values = vec![
        245, 378, 312, 489, 645, 699, 945, 1189, 1056, 978, 743, 892, 1056, 1211,
    ];

    view! {
        <div class="bg-[#0a1428] rounded-3xl border border-slate-700 p-4 h-full">
            <div class="flex h-full gap-2">

                <div class="flex flex-col justify-between text-xs text-slate-400 py-1 w-fit text-left">
                    <p>{max_value}</p>
                    <p>{max_value / 2}</p>
                    <span>"0"</span>
                </div>

                <div class="flex-1 flex flex-col min-w-0">

                    <div class="relative flex-1">

                        <div class="absolute inset-0 flex flex-col justify-between pointer-events-none">
                            <div class="border-t border-slate-700/70"></div>
                            <div class="border-t border-slate-700/70"></div>
                            <div class="border-t border-slate-700/70"></div>
                        </div>

                        <div class="absolute inset-0 flex items-stretch gap-1">
                            {
                                values.iter().map(|value| {
                                    let height =
                                        ((*value as f32 / max_value as f32) * 100.0).max(8.0);

                                    view! {
                                        <div class="flex-1 flex flex-col justify-end items-center h-full">
                                            <div
                                                class="w-full rounded-t bg-cyan-200
                                                       hover:bg-cyan-500 transition-all duration-300
                                                       shadow-[0_0_15px_#a4f3fc]
                                                       hover:shadow-[0_0_20px_#1996af]"
                                                style:height=format!("{height}%")
                                            />
                                        </div>
                                    }
                                }).collect_view()
                            }
                        </div>
                    </div>

                    // Labels
                    <div class="mt-3 flex gap-2">
                        {
                            day_labels.iter().map(|label| {
                                view! {
                                    <div class="flex-1 text-center text-[8px] text-slate-400">
                                        {label.clone()}
                                    </div>
                                }
                            }).collect_view()
                        }
                    </div>

                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LineChartMock() -> impl IntoView {
    let today = Local::now().date_naive();

    let labels: Vec<String> = (0..14)
        .rev()
        .map(|i| {
            (today - chrono::Duration::days(i))
                .format("%m/%d")
                .to_string()
        })
        .collect();

    let values = vec![
        245.0, 378.0, 312.0, 489.0, 645.0, 699.0, 945.0, 1189.0, 1056.0, 978.0, 743.0, 892.0,
        1056.0, 1211.0,
    ];

    let max = 1400.0;

    const VIEWBOX_WIDTH: f32 = 100.0;
    const VIEWBOX_HEIGHT: f32 = 100.0;

    let padding = 8.0;

    let step = (VIEWBOX_WIDTH - padding * 2.0) / (values.len() - 1) as f32;

    let points: Vec<(f32, f32)> = values
        .iter()
        .enumerate()
        .map(|(i, value)| {
            let x = padding + step * i as f32;

            let y = VIEWBOX_HEIGHT - padding - ((*value / max) * (VIEWBOX_HEIGHT - padding * 2.0));

            (x, y)
        })
        .collect();

    let path = points
        .iter()
        .enumerate()
        .map(|(i, (x, y))| {
            if i == 0 {
                format!("M {} {}", x, y)
            } else {
                format!(" L {} {}", x, y)
            }
        })
        .collect::<String>();

    view! {

        <div class="bg-[#0D1628] rounded-[28px] border border-[#223149] p-2 h-full">

            <div class="flex gap-4 h-full">

                <div class="flex flex-col justify-between text-xs text-slate-500 py-2">

                    <span>{max as i32}</span>

                    <span>{(max / 2.0) as i32}</span>

                    <span>"0"</span>

                </div>

                <div class="flex-1 flex flex-col min-w-0">

                    <div class="relative flex-1 aspect-[16/5] ">

                            <svg
                                class="absolute inset-0 w-full h-full"
                                viewBox="0 0 100 100"
                                preserveAspectRatio="none"
                            >
                            <defs>

                                <linearGradient id="line-gradient" x1="0%" x2="100%">
                                    <stop offset="0%" stop-color="#7DE8FF"/>
                                    <stop offset="100%" stop-color="#18CFF8"/>
                                </linearGradient>

                                <filter id="glow">

                                    <feGaussianBlur stdDeviation="4" result="blur"/>

                                    <feMerge>
                                        <feMergeNode in="blur"/>
                                        <feMergeNode in="SourceGraphic"/>
                                    </feMerge>

                                </filter>

                            </defs>

                            <line x1="0" y1="12" x2="1000" y2="12"
                                stroke="#24364E"
                                stroke-dasharray="4 6"/>

                            <line x1="0" y1="58" x2="1000" y2="58"
                                stroke="#24364E"
                                stroke-dasharray="4 6"/>

                            <path
                                d=path
                                fill="none"
                                stroke="url(#line-gradient)"
                                stroke-width="1"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                filter="url(#glow)"
                            />

                            {
                                points.iter().map(|(x,y)|{

                                    view!{

                                        <>
                                            <circle
                                                cx=*x
                                                cy=*y
                                                r="2"
                                                fill="#18CFF8"
                                                opacity="0.18"
                                            />

                                            <circle
                                                cx=*x
                                                cy=*y
                                                r="1"
                                                fill="#8EF2FF"
                                            />
                                        </>

                                    }

                                }).collect_view()
                            }

                        </svg>

                    </div>

                    <div
                        class="mt-1 grid"
                        style=format!("grid-template-columns: repeat({}, minmax(0, 1fr));", labels.len())
                    >
                        {
                            labels.iter().map(|label| {
                                view! {
                                    <span class="text-center text-[8px] text-slate-500">
                                        {label.clone()}
                                    </span>
                                }
                            }).collect_view()
                        }
                    </div>
                </div>

            </div>

        </div>

    }
}
