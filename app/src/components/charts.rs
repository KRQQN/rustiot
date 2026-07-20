use chrono::Local;
use leptos::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn LineChart(
    #[prop(into)] values: Signal<Vec<f32>>,

    #[prop(into, default = Signal::derive(|| 1400.0f32))] max: Signal<f32>,

    #[prop(into, default = None)] title: Option<Signal<String>>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };

        let rect = canvas.get_bounding_client_rect();
        let width = rect.width() as u32;
        let height = rect.height() as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

        let vals = values.get();
        if vals.is_empty() {
            return;
        }

        let max_val = max.get() as f64;
        let padding_left = 50.0f64;
        let padding_bottom = 45.0f64;
        let w = width as f64;
        let h = height as f64;

        ctx.set_fill_style(&"#94a3b8".into());
        ctx.set_font("8px sans-serif");
        ctx.set_text_align("right");

        for i in 0..=2 {
            let value = max_val * (i as f64 / 2.0);
            let y = h - padding_bottom - (value / max_val) * (h - padding_bottom - 30.0);

            ctx.set_stroke_style(&"#24364E".into());
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(padding_left - 10.0, y);
            ctx.line_to(w - 20.0, y);
            ctx.stroke();

            ctx.fill_text(&value.round().to_string(), padding_left - 15.0, y + 4.0)
                .ok();
        }

        ctx.set_stroke_style(&"#18CFF8".into());
        ctx.set_line_width(3.5);
        ctx.set_line_join("round");
        ctx.set_line_cap("round");

        ctx.set_shadow_color("#18CFF8");
        ctx.set_shadow_blur(15.0);

        ctx.begin_path();
        for (i, &value) in vals.iter().enumerate() {
            let x = padding_left
                + (w - padding_left - 30.0) * (i as f64 / (vals.len().max(2) - 1) as f64);
            let y = h - padding_bottom - (value as f64 / max_val) * (h - padding_bottom - 30.0);

            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();

        ctx.set_shadow_blur(0.0);
        for (i, &value) in vals.iter().enumerate() {
            let x = padding_left
                + (w - padding_left - 30.0) * (i as f64 / (vals.len().max(2) - 1) as f64);
            let y = h - padding_bottom - (value as f64 / max_val) * (h - padding_bottom - 30.0);

            ctx.set_fill_style(&"#18CFF8".into());
            ctx.begin_path();
            ctx.arc(x, y, 5.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();

            ctx.set_fill_style(&"#ffffff".into());
            ctx.begin_path();
            ctx.arc(x, y, 2.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.fill();
        }

        ctx.set_fill_style(&"#94a3b8".into());
        ctx.set_font("8px sans-serif");
        ctx.set_text_align("center");

        let today = Local::now().date_naive();
        for (i, _) in vals.iter().enumerate() {
            let date = today - chrono::Duration::days((vals.len() - 1 - i) as i64);
            let label = date.format("%m/%d").to_string();

            let x = padding_left
                + (w - padding_left - 30.0) * (i as f64 / (vals.len().max(2) - 1) as f64);
            let y = h - padding_bottom + 25.0;

            ctx.fill_text(&label, x, y).ok();
        }
    });

    view! {
        <div class="bg-[#0D1628] rounded-[28px] border border-[#223149] p-1 h-full flex flex-col">
            {title.map(|t| view! { <div class="text-sm  mb-4">{t}</div> })}

            <div class="flex-1 min-h-0 relative">
                <canvas
                    node_ref=canvas_ref
                    class="w-full h-full rounded-xl"
                />
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
