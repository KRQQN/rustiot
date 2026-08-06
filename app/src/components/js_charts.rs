#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]
use crate::e_charts::EChart;
use leptos::html;
use leptos::prelude::*;
use serde_json::json;
use serde_json::Value;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::ResizeObserver;

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
                                { "offset": 0, "color": "rgba(103, 232, 249, 0.5)" },
                                { "offset": 1, "color": "rgba(165, 243, 252, 0)" }
                            ]
                        }
                    },
                    "lineStyle": {
                        "width": 3,
                        "shadowBlur": 15,
                        "color": "rgba(103, 232, 249, 0.8)"
                    }
                }]
            });

            let js = js_sys::JSON::parse(&options.to_string()).unwrap();

            chart.with_value(|slot| {
                if let Some(c) = slot {
                    c.update(&js);
                }
            });
        });

        Effect::new(move |_| {
            let Some(div) = node.get() else {
                return;
            };

            let cb = Closure::wrap(Box::new(
                move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                    chart.with_value(|slot| {
                        if let Some(c) = slot {
                            c.resize();
                        }
                    });
                },
            )
                as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);

            let observer = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();

            observer.observe(&div);

            cb.forget();

            on_cleanup(move || {
                observer.disconnect();
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
        // Create the chart once when the DOM node is ready
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

        //  Update options whenever labels / values change
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
                                { "offset": 0, "color": "rgba(103, 232, 249, 0.7)" },
                                { "offset": 1, "color": "rgba(165, 243, 252, 0.1)" }
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
                if let Some(c) = slot {
                    c.update(&js);
                }
            });
        });

        // ResizeObserver (!) makes the chart follow the panel size
        Effect::new(move |_| {
            let Some(div) = node.get() else {
                return;
            };

            let cb = Closure::wrap(Box::new(
                move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                    chart.with_value(|slot| {
                        if let Some(c) = slot {
                            c.resize();
                        }
                    });
                },
            )
                as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);

            let observer = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();

            observer.observe(&div);

            // Forget the closure so it stays alive (standard pattern)
            cb.forget();

            // Still disconnect the observer when the component is destroyed
            on_cleanup(move || {
                observer.disconnect();
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

// ############ For calendar heat map ##############

// let year = Signal::derive(|| "2026".to_string());
//
// let cdata = Signal::derive(|| {
//     vec![
//         ("2026-01-01".to_string(), 4210.0),
//         ("2026-01-03".to_string(), 8123.0),
//         ("2026-01-05".to_string(), 6750.0),
//         ("2026-01-08".to_string(), 9340.0),
//         ("2026-01-10".to_string(), 2860.0),
//         ("2026-01-12".to_string(), 7312.0),
//         ("2026-01-15".to_string(), 9980.0),
//         ("2026-01-18".to_string(), 4521.0),
//         ("2026-01-20".to_string(), 6145.0),
//         ("2026-01-23".to_string(), 8470.0),
//         ("2026-01-26".to_string(), 2150.0),
//         ("2026-01-29".to_string(), 7034.0),
//         ("2026-02-01".to_string(), 3890.0),
//         ("2026-02-03".to_string(), 5480.0),
//         ("2026-02-05".to_string(), 9210.0),
//         ("2026-02-08".to_string(), 6675.0),
//         ("2026-02-10".to_string(), 4820.0),
//         ("2026-02-12".to_string(), 10320.0),
//         ("2026-02-15".to_string(), 7560.0),
//         ("2026-02-18".to_string(), 3200.0),
//         ("2026-02-20".to_string(), 8445.0),
//         ("2026-02-22".to_string(), 9120.0),
//         ("2026-02-25".to_string(), 5780.0),
//         ("2026-02-28".to_string(), 6420.0),
//         ("2026-03-01".to_string(), 7120.0),
//         ("2026-03-03".to_string(), 9850.0),
//         ("2026-03-05".to_string(), 4360.0),
//         ("2026-03-08".to_string(), 8750.0),
//         ("2026-03-10".to_string(), 2920.0),
//         ("2026-03-12".to_string(), 5640.0),
//         ("2026-03-15".to_string(), 11040.0),
//         ("2026-03-18".to_string(), 6870.0),
//         ("2026-03-20".to_string(), 4555.0),
//         ("2026-03-23".to_string(), 8090.0),
//         ("2026-03-26".to_string(), 9735.0),
//         ("2026-03-29".to_string(), 6150.0),
//         ("2026-03-31".to_string(), 7425.0),
//     ]
// });

#[component]
pub fn ECalendarHeatmap(
    #[prop(into)] values: Signal<Vec<(String, f64)>>,
    #[prop(into)] year: Signal<String>,
) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let chart = StoredValue::new(None::<EChart>);

    #[cfg(not(feature = "ssr"))]
    {
        // 1. Create the chart once
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

        // 2. Update options whenever data changes
        Effect::new(move |_| {
            let options = json!({
                "backgroundColor": "transparent",
                "tooltip": {},
                "visualMap": {
                    "min": 0,
                    "max": 10000,
                    "type": "piecewise",
                    "orient": "horizontal",
                    "left": "center",
                    "top": 20
                },
                "calendar": {
                    "top": 80,
                    "left": 20,
                    "right": 20,
                    "cellSize": ["auto", 13],
                    "range": year.get(),
                    "itemStyle": {
                        "borderWidth": 0.5
                    },
                    "yearLabel": {
                        "show": false
                    }
                },
                "series": [{
                    "type": "heatmap",
                    "coordinateSystem": "calendar",
                    "data": values.get()
                }]
            });

            let js = js_sys::JSON::parse(&options.to_string()).unwrap();

            chart.with_value(|slot| {
                if let Some(c) = slot {
                    c.update(&js);
                }
            });
        });

        // 3. ResizeObserver
        Effect::new(move |_| {
            let Some(div) = node.get() else {
                return;
            };

            let cb = Closure::wrap(Box::new(
                move |_entries: js_sys::Array, _observer: web_sys::ResizeObserver| {
                    chart.with_value(|slot| {
                        if let Some(c) = slot {
                            c.resize();
                        }
                    });
                },
            )
                as Box<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>);

            let observer = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();

            observer.observe(&div);

            // Forget the closure (standard pattern)
            cb.forget();

            on_cleanup(move || {
                observer.disconnect();
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
pub fn ETreeChart(#[prop(into)] data: Signal<Value>) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let chart = StoredValue::new(None::<EChart>);

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
                            "series": [{
                "type": "tree",

                "data": [data.get()],

                "left": "2%",
                "right": "2%",
                "top": "20%",
                "bottom": "8%",

                "orient": "BT",

                "initialTreeDepth": -1,

                "symbol": "emptyCircle",
                "symbolSize": 8,

                "expandAndCollapse": true,

                "label": {
                    "position": "bottom",
                    "rotate": 90,
                    "verticalAlign": "left",
                    "align": "right",
                    "fontSize": 16,
                    "textBorderWidth": 0,
                    "textBorderColor": "transparent",
                    "color": "#fff",
                    "rotate": 0,
                },

                "leaves": {
                    "label": {
                        "position": "top",
                        "textBorderWidth": 0,
                        "textBorderColor": "transparent",
                        "align":"left",
                        "verticalAlign": "middle",
                        "color": "#fff",
                        "rotate": 90,
                        "right": "10%",
                        "fontSize": 16

                    }
                },

                "lineStyle": {
                    "width": 2,
                    "curveness": 0.5,
                    "color": "rgba(6, 182, 212, 0.8)",
                },

                "emphasis": {
                    "focus": "descendant"
                },

                "animationDuration": 550,
                "animationDurationUpdate": 750
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
