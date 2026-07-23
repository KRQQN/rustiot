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
                    "trigger": "axis"
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
                    "reverse": true,
                },

                "yAxis": {
                    "type": "value",
                    "max": max.get(),
                },

                "series": [{
                    "type": "line",
                    "smooth": true,
                    "data": values.get(),
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
