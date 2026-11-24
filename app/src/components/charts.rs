use leptos::prelude::*;

#[component]
pub fn NetworkChart() -> impl IntoView {
    let nodes = [
        (50, 50),
        (150, 70),
        (260, 45),
        (90, 170),
        (210, 170),
        (150, 260),
    ];

    let edges = [
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

    let markers = [(40, 250), (120, 180), (210, 140), (260, 70)];

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
