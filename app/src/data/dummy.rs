use crate::components::chart::Chart;
use crate::components::dashboard::panel::{Panel, PanelConfig, PanelContent};
use crate::components::dashboard::panel_placer::PanelLayout;
use crate::components::js_charts::ECalendarHeatmap;
use leptos::prelude::*;

use leptos::reactive::wrappers::read::Signal;

pub fn calendar_heatmap_data() -> Vec<(String, f64)> {
    vec![
        ("2026-01-01".into(), 4210.0),
        ("2026-01-03".into(), 8123.0),
        ("2026-01-05".into(), 6750.0),
        ("2026-01-08".into(), 9340.0),
        ("2026-01-10".into(), 2860.0),
        ("2026-01-12".into(), 7312.0),
        ("2026-01-15".into(), 9980.0),
        ("2026-01-18".into(), 4521.0),
        ("2026-01-20".into(), 6145.0),
        ("2026-01-23".into(), 8470.0),
        ("2026-01-26".into(), 2150.0),
        ("2026-01-29".into(), 7034.0),
        ("2026-02-01".into(), 3890.0),
        ("2026-02-03".into(), 5480.0),
        ("2026-02-05".into(), 9210.0),
        ("2026-02-08".into(), 6675.0),
        ("2026-02-10".into(), 4820.0),
        ("2026-02-12".into(), 10320.0),
        ("2026-02-15".into(), 7560.0),
        ("2026-02-18".into(), 3200.0),
        ("2026-02-20".into(), 8445.0),
        ("2026-02-22".into(), 9120.0),
        ("2026-02-25".into(), 5780.0),
        ("2026-02-28".into(), 6420.0),
        ("2026-03-01".into(), 7120.0),
        ("2026-03-03".into(), 9850.0),
        ("2026-03-05".into(), 4360.0),
        ("2026-03-08".into(), 8750.0),
        ("2026-03-10".into(), 2920.0),
        ("2026-03-12".into(), 5640.0),
        ("2026-03-15".into(), 11040.0),
        ("2026-03-18".into(), 6870.0),
        ("2026-03-20".into(), 4555.0),
        ("2026-03-23".into(), 8090.0),
        ("2026-03-26".into(), 9735.0),
        ("2026-03-29".into(), 6150.0),
        ("2026-03-31".into(), 7425.0),
    ]
}

#[cfg(not(feature = "ssr"))]
pub async fn db_read_get_panels() -> Vec<Panel> {
    vec![
        Panel {
            id: "1".into(),
            title: "Liters".into(),
            content: ViewFn::from(move || {
                view! {
                    <Chart device_id="esp-rustiot".to_string() timespan="24h".to_string() chart_type=PanelContent::BarChart/>
                }
                .into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        },
        Panel {
            id: "2".into(),
            title: "Temperature".into(),
            content: ViewFn::from(move || {
                view! {
                    <Chart device_id="esp-rustiot".to_string() timespan="24h".to_string() chart_type=PanelContent::LineChart/>
                }
                .into_any()
            }),
            layout: PanelLayout {
                x: 360.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        },
        Panel {
            id: "3".into(),
            title: "Usage calendar".into(),
            content: ViewFn::from(move || {
                view! {
                    // TODO  CalendarHeatmap -> Low priority
                }
                .into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 260.0,
                w: 660.0,
                h: 180.0,
            },
        },
    ]
}

pub fn new_panel(cfg: PanelConfig, id: String) -> Panel {
    let source_label = match cfg.data_source.as_str() {
        "esp-rustiot" => "ESP32",
        "device-2" => "Phone",
        "device-3" => "Workstation",
        other => other,
    };
    let timespan_label = match cfg.timespan.as_str() {
        "24h" => "24h",
        "7d" => "7d",
        "30d" => "30d",
        "90d" => "90d",
        other => other,
    };

    let device_id = cfg.data_source.clone();
    let timespan = cfg.timespan.clone();

    match cfg.panel_type {
        PanelContent::BarChart => Panel {
            id,
            title: format!("Temperature · {source_label} · {timespan_label}").into(),
            content: ViewFn::from(move || {
                view! {
                    <Chart
                        device_id=device_id.clone()
                        timespan=timespan.clone()
                        chart_type=PanelContent::BarChart
                    />
                }
                .into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        },
        PanelContent::LineChart => Panel {
            id,
            title: format!("Temperature · {source_label} · {timespan_label}").into(),
            content: ViewFn::from(move || {
                view! {
                    <Chart
                        device_id=device_id.clone()
                        timespan=timespan.clone()
                        chart_type=PanelContent::LineChart
                    />
                }
                .into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        },
        PanelContent::CalendarHeatmap => {
            // still dummy for now – same pattern later when you have a calendar Chart
            let cdata = Signal::derive(calendar_heatmap_data);
            let year = Signal::derive(|| "2026".to_string());
            Panel {
                id,
                title: format!("Calendar · {source_label} · {timespan_label}").into(),
                content: ViewFn::from(move || {
                    view! {
                        <ECalendarHeatmap values=cdata year=year />
                    }
                    .into_any()
                }),
                layout: PanelLayout {
                    x: 20.0,
                    y: 20.0,
                    w: 660.0,
                    h: 180.0,
                },
            }
        }
    }
}
