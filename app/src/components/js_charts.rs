#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]
use crate::e_charts::EChart;
use leptos::html;
use leptos::prelude::*;
use serde_json::json;
use wasm_bindgen::JsValue;

#[component]
pub fn ELineChart(
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
                    "splitLine": {
                        "lineStyle": {
                            "color": "rgba(255,255,255,0.08)",
                            "type": "dashed"
                        }
                    }
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
                                { "offset": 0, "color": "rgba(103, 232, 249, 0.5)"},
                                { "offset": 1, "color": "rgba(165, 243, 252, 0)"}
                            ]
                        }
                    },

                    "lineStyle": {
                        "width": 3,
                        "shadowBlur": 15,
                        "color": "rgba(103, 232, 249, 0.8)"
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

#[component]
pub fn EBarChart(
    #[prop(into)] labels: Signal<Vec<String>>,
    #[prop(into)] values: Signal<Vec<f64>>,
) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let chart = StoredValue::new(None::<EChart>);
    let max = Signal::derive(move || values.get().iter().copied().fold(0.0, f64::max));

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            let Some(div) = node.get() else {
                return;
            };

            chart.update_value(|slot| {
                if slot.is_none() {
                    let js = js_sys::JSON::parse("{}").unwrap();
                    *slot = Some(EChart::new(div, &js));
                }
            });
        });

        Effect::new(move |_| {
            let options = json!({
                "backgroundColor": "transparent",

                "tooltip": {
                    "trigger": "item",
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
                    "data": labels.get(),
                    "axisTick": {
                        "show": false
                    }
                },

                "yAxis": {
                    "type": "value",
                    "max": max.get(),
                    "splitLine": {
                        "lineStyle": {
                            "color": "rgba(255,255,255,0.08)",
                            "type": "dashed"
                        }
                    }
                },

                "series": [{
                    "type": "bar",
                    "data": values.get(),
                    "barWidth": "55%",

                    "itemStyle": {
                        "borderRadius": [8, 8, 0, 0],
                        "color": {
                            "type": "linear",
                            "x": 0,
                            "y": 0,
                            "x2": 0,
                            "y2": 1,
                            "colorStops": [
                                { "offset": 0, "color": "rgba(103, 232, 249, 0.7)"},
                                { "offset": 1, "color": "rgba(165, 243, 252, 0.1)"}
                            ]
                        },
                        "shadowBlur": 12,
                        "shadowColor": "rgba(6, 182, 212, 1)"
                    },

                    "emphasis": {
                        "focus": "self",
                        "itemStyle": {
                            "color": "rgba(6, 182, 212, 1)"
                        }
                    },

                    "blur": {
                        "itemStyle": {
                            "opacity": 0.6
                        }
                    }
                }]
            });

            let js = js_sys::JSON::parse(&options.to_string()).unwrap();

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
