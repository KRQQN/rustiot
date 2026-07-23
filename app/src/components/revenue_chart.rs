#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]
use crate::e_charts::EChart;
use leptos::html;
use leptos::prelude::*;
use serde_json::json;
use wasm_bindgen::JsValue;

#[component]
pub fn RevenueChart(
    #[prop(into)] labels: Signal<Vec<String>>,
    #[prop(into)] values: Signal<Vec<f64>>,
) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let chart = StoredValue::new(None::<EChart>);
    let max = Signal::derive(move || values.get().iter().copied().fold(0.0, f64::max));

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::logging;

        // Create once
        Effect::new(move |_| {
            let Some(div) = node.get() else {
                return;
            };

            chart.update_value(|slot| {
                if slot.is_none() {
                    let options = json!({});
                    let text = options.to_string();
                    let js = js_sys::JSON::parse(&text).unwrap();

                    *slot = Some(EChart::new(div, &js));
                }
            });
        });

        Effect::new(move |_| {
            let options = json!({
                "backgroundColor": "transparent",

                "tooltip": {
                    "trigger": "axis",
                    "backgroundColor": "rgba(30,30,30,0.85)",
                    "borderRadius": 12,
                    "borderWidth": 0,
                    "padding": 12,
                    "textStyle": {
                        "fontSize": 14,
                        "color": "#fff"
                    }
                },

                "grid": {
                    "left": 50,
                    "right": 20,
                    "top": 30,
                    "bottom": 40
                },

                "xAxis": {
                    "type": "category",
                    "boundaryGap": false,
                    "data": labels.get(),
                },

                "yAxis": {
                    "type": "value",
                    "max": max.get(),
                },

                "series": [{
                    "type": "line",
                    "smooth": true,
                    "data": values.get(),
                    "areaStyle": {
                        "color": {
                            "type": "linear",
                            "x": 0,
                            "y": 0,
                            "x2": 0,
                            "y2": 1,
                            "colorStops": [
                            { "offset": 0, "color": "rgba(99,102,241,0.4)" },
                                { "offset": 1, "color": "rgba(99,102,241,0)" }
                            ]
                        }
                    },
                    "lineStyle": {
                        "width": 3,
                        "shadowBlur": 15,
                        "shadowColor": "rgba(99,102,241,0.7)"
                    },

                }]
            });

            let text = options.to_string();
            let js = js_sys::JSON::parse(&text).unwrap();

            chart.with_value(|slot| {
                if let Some(chart) = slot {
                    chart.update(&js);
                }
            });
        });
    }

    view! {
        <div
            node_ref=node
            class="w-full h-full"
        />
    }
}
