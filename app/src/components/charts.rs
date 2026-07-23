use chrono::{DateTime, Local};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlElement};

pub struct DateValuePoint {
    pub date: DateTime<Local>,
    pub value: f64,
}

#[component]
pub fn BarChart(
    #[prop(into)] values: Signal<Vec<f64>>,
    #[prop(into, default = None)] title: Option<Signal<String>>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let hovered = RwSignal::new(None::<usize>);
    let max = Signal::derive(move || values.get().iter().copied().fold(0.0, f64::max));

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };

        let dpr = web_sys::window().unwrap().device_pixel_ratio();
        let rect = canvas.get_bounding_client_rect();
        let logical_width = rect.width();
        let logical_height = rect.height();

        // internal canvas resolution
        let width = (logical_width * dpr) as u32;
        let height = (logical_height * dpr) as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        let html: &HtmlElement = canvas.as_ref();

        let _ = html
            .style()
            .set_property("height", &format!("{}px", logical_height));

        let _ = html
            .style()
            .set_property("width", &format!("{}px", logical_width));

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.scale(dpr, dpr).unwrap();

        ctx.clear_rect(0.0, 0.0, rect.width(), rect.height());

        let vals = values.get();
        if vals.is_empty() {
            return;
        }

        let max_val = max.get();
        let padding_left = 48.0;
        let padding_bottom = 45.0;
        let w = rect.width();
        let h = rect.height();

        let bar_count = vals.len() as f64;
        let gap = 8.0;
        let bar_width = ((w - padding_left - 30.0) / bar_count) - gap;

        // Y labels + grid
        ctx.set_font("13px Inter, sans-serif");
        ctx.set_text_align("right");
        for i in 0..=2 {
            let val = (max_val * (i as f64 / 2.0)) as i32;
            let y = h - padding_bottom - (val as f64 / max_val) * (h - padding_bottom - 35.0);

            ctx.set_stroke_style_str("#24364E");
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(padding_left - 12.0, y);
            ctx.line_to(w - 20.0, y);
            ctx.stroke();

            ctx.set_fill_style_str("#94a3b8");
            ctx.fill_text(&val.to_string(), padding_left - 18.0, y + 4.0)
                .ok();
        }

        // Bars
        for (i, &value) in vals.iter().enumerate() {
            let height = ((value / max_val) * (h - padding_bottom - 35.0)).max(12.0);
            let x = padding_left + (i as f64) * (bar_width + gap);
            let y = h - padding_bottom - height;

            let is_hovered = hovered.get() == Some(i);

            ctx.set_shadow_color(if is_hovered { "#a5f3fc" } else { "#67e8f9" });
            ctx.set_shadow_blur(if is_hovered { 20.0 } else { 10.0 });

            ctx.set_fill_style_str(if is_hovered { "#a5f3fc" } else { "#67e8f9" });
            ctx.fill_rect(x, y, bar_width, height);

            ctx.set_shadow_blur(0.0);
        }

        // X labels
        ctx.set_fill_style_str("#94a3b8");
        ctx.set_font("10.5px Inter, sans-serif");
        ctx.set_text_align("center");

        let today = Local::now().date_naive();
        for (i, _) in vals.iter().enumerate() {
            let date = today - chrono::Duration::days((vals.len() - 1 - i) as i64);
            let label = date.format("%m/%d").to_string();
            let x = padding_left + (i as f64) * (bar_width + gap) + bar_width / 2.0;
            ctx.fill_text(&label, x, h - padding_bottom + 28.0).ok();
        }
    });

    let on_mousemove = move |ev: web_sys::MouseEvent| {
        if let Some(canvas) = canvas_ref.get() {
            let rect = canvas.get_bounding_client_rect();
            let _x = ev.client_x() as f64 - rect.left();
        }
    };

    view! {
        <div class="bg-[#0a1428] rounded-3xl border border-slate-700 p-4 h-full flex flex-col">
            {title.map(|t| view! { <div class="text-lg font-semibold text-white mb-4 px-1">{t}</div> })}

            <div class="flex-1 min-h-0 relative">
                <canvas
                    node_ref=canvas_ref
                    class="w-full h-full rounded-xl"
                    on:mousemove=on_mousemove
                />
            </div>
        </div>
    }
}

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
