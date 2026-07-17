use chrono::Local;
use leptos::prelude::*;

#[component]
pub fn LineChart() -> impl IntoView {
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

    let width = 1000.0;
    let height = 300.0;
    let padding = 20.0;

    let step = (width - padding * 2.0) / (values.len() - 1) as f32;

    let points: Vec<(f32, f32)> = values
        .iter()
        .enumerate()
        .map(|(i, value)| {
            let x = padding + step * i as f32;

            let y = height - padding - ((*value / max) * (height - padding * 2.0));

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

        <div class="bg-[#0D1628] rounded-[28px] border border-[#223149] p-5 h-full">

            <div class="flex gap-4 h-full">

                <div class="flex flex-col justify-between text-xs text-slate-500 py-2">

                    <span>{max as i32}</span>

                    <span>{(max / 2.0) as i32}</span>

                    <span>"0"</span>

                </div>

                <div class="flex-1 flex flex-col min-w-0">

                    <div class="relative flex-1">

                        <svg
                            class="w-full h-full overflow-visible"
                            viewBox="0 0 1000 300"
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

                            <line x1="0" y1="20" x2="1000" y2="20"
                                stroke="#24364E"
                                stroke-dasharray="4 6"/>

                            <line x1="0" y1="150" x2="1000" y2="150"
                                stroke="#24364E"
                                stroke-dasharray="4 6"/>

                            <line x1="0" y1="280" x2="1000" y2="280"
                                stroke="#24364E"
                                stroke-dasharray="4 6"/>

                            <path
                                d=path
                                fill="none"
                                stroke="url(#line-gradient)"
                                stroke-width="4"
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
                                                r="8"
                                                fill="#18CFF8"
                                                opacity="0.18"
                                            />

                                            <circle
                                                cx=*x
                                                cy=*y
                                                r="4"
                                                fill="#8EF2FF"
                                            />
                                        </>

                                    }

                                }).collect_view()
                            }

                        </svg>

                    </div>

                    <div
                        class="mt-4 grid"
                        style=format!("grid-template-columns: repeat({}, minmax(0, 1fr));", labels.len())
                    >
                        {
                            labels.iter().map(|label| {
                                view! {
                                    <span class="text-center text-[9px] text-slate-500">
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

#[component]
pub fn BarChart() -> impl IntoView {
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
pub fn NetworkChart() -> impl IntoView {
    let nodes = vec![
        (50, 50),
        (150, 70),
        (260, 45),
        (90, 170),
        (210, 170),
        (150, 260),
    ];

    let edges = vec![
        (0, 1),
        (1, 2),
        (0, 3),
        (1, 3),
        (1, 4),
        (2, 4),
        (3, 5),
        (4, 5),
    ];

    view! {

        <div class="bg-[#0a1428] rounded-3xl border border-slate-700 p-4 h-full">

            <svg
                class="w-full h-full"
                viewBox="0 0 300 300"
                preserveAspectRatio="xMidYMid meet"
            >

                <defs>

                    <filter id="network-glow">

                        <feGaussianBlur stdDeviation="3" result="blur"/>

                        <feMerge>
                            <feMergeNode in="blur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>

                    </filter>

                </defs>

                {
                    edges.iter().map(|(a,b)|{

                        let (x1,y1)=nodes[*a];
                        let (x2,y2)=nodes[*b];

                        view!{

                            <line
                                x1=x1
                                y1=y1
                                x2=x2
                                y2=y2
                                stroke="#29405E"
                                stroke-width="2"
                            />

                        }

                    }).collect_view()
                }

                {
                    nodes.iter().map(|(x,y)|{

                        view!{

                            <>
                                <circle
                                    cx=*x
                                    cy=*y
                                    r="10"
                                    fill="#18CFF8"
                                    opacity="0.15"
                                    filter="url(#network-glow)"
                                />

                                <circle
                                    cx=*x
                                    cy=*y
                                    r="5"
                                    fill="#9EEFFF"
                                />
                            </>

                        }

                    }).collect_view()
                }

            </svg>

        </div>

    }
}

#[component]
pub fn TrackingChart() -> impl IntoView {
    let path = "M40 250
                C80 210 120 170 170 180
                S250 120 260 70";

    let markers = vec![(40, 250), (120, 180), (210, 140), (260, 70)];

    view! {

        <div class="bg-[#0a1428] rounded-3xl border border-slate-700 p-4 h-full">

            <svg
                class="w-full h-full"
                viewBox="0 0 300 300"
                preserveAspectRatio="xMidYMid meet"
            >

                <defs>

                    <filter id="tracking-glow">

                        <feGaussianBlur stdDeviation="4" result="blur"/>

                        <feMerge>
                            <feMergeNode in="blur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>

                    </filter>

                </defs>

                <path
                    d=path
                    fill="none"
                    stroke="#18CFF8"
                    stroke-width="4"
                    stroke-linecap="round"
                    filter="url(#tracking-glow)"
                    stroke-dasharray="8 8"
                />

                {
                    markers.iter().enumerate().map(|(i,(x,y))|{

                        let r = if i==markers.len()-1 {7}else{5};

                        view!{

                            <>
                                <circle
                                    cx=*x
                                    cy=*y
                                    r=r+6
                                    fill="#18CFF8"
                                    opacity="0.12"
                                />

                                <circle
                                    cx=*x
                                    cy=*y
                                    r=r
                                    fill="#A4F4FF"
                                />
                            </>

                        }

                    }).collect_view()
                }

            </svg>

        </div>

    }
}
