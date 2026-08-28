use crate::components::chart::Chart;
use crate::components::charts::DateValuePoint;
use crate::components::dashboard::panel::{Panel, PanelConfig, PanelContent};
use crate::components::dashboard::panel_placer::PanelLayout;
use crate::components::js_charts::{ECalendarHeatmap, ELineChart};
use crate::server_fns::get_device_data::DeviceReading;
use chrono::{Duration, Local};
use leptos::prelude::*;

use leptos::reactive::wrappers::read::Signal;
#[cfg(not(feature = "ssr"))]
use rand::Rng;

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

pub fn chart_data() -> Vec<DateValuePoint> {
    let start = Local::now();

    #[cfg(feature = "ssr")]
    {
        (0..10)
            .map(|i| DateValuePoint {
                date: start - Duration::days(i),
                value: 30.0 + (i as f64 * 15.0),
            })
            .collect()
    }

    #[cfg(not(feature = "ssr"))]
    {
        let mut rng = rand::thread_rng();

        (0..10)
            .map(|i| DateValuePoint {
                date: start - Duration::days(i),
                value: (rng.gen_range(0.0_f64..=100.0_f64) * 10.0).round() / 10.0,
            })
            .collect()
    }
}

#[server(GetDummyDeviceData)]
pub async fn get_device_data(
    device_id: String,
    _timespan: String,
    _limit: Option<u32>,
) -> Result<Vec<DeviceReading>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use chrono::{Duration, Utc};

        let now = Utc::now();

        let readings = (0..15)
            .map(|i| {
                // Spread 15 points evenly across the last 10 days.
                let offset_days = (i as f64) * 10.0 / 14.0;
                let time = now - Duration::seconds((offset_days * 24.0 * 60.0 * 60.0) as i64);

                DeviceReading {
                    time: time.to_rfc3339(),
                    device_id: device_id.clone(),

                    humidity: 45.0 + (i as f64 * 1.7),
                    temperature: 20.0 + (i as f64 * 0.5),
                }
            })
            .collect();

        Ok(readings)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

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

// pub fn new_panel(panel_type: PanelContent, id: String) -> Panel {
//     let chart_data = chart_data();
//
//     let mut labels = chart_data
//         .iter()
//         .map(|p| p.date.format("%d %b").to_string())
//         .collect::<Vec<_>>();
//     labels.reverse();
//
//     let mut chart_values = chart_data.iter().map(|p| p.value).collect::<Vec<_>>();
//     chart_values.reverse();
//
//     let labels = Signal::derive(move || labels.clone());
//     let chart_values = Signal::derive(move || chart_values.clone());
//
//     let cdata = Signal::derive(calendar_heatmap_data);
//     let year = Signal::derive(|| "2026".to_string());
//
//     match panel_type {
//         PanelContent::BarChart => Panel {
//             id,
//             title: "Bar Chart".into(),
//             content: ViewFn::from(move || {
//                 view! {
//                     <EBarChart
//                         values=chart_values
//                         labels=labels
//                     />
//                 }
//                 .into_any()
//             }),
//             layout: PanelLayout {
//                 x: 20.0,
//                 y: 20.0,
//                 w: 320.0,
//                 h: 220.0,
//             },
//         },
//
//         PanelContent::LineChart => Panel {
//             id,
//             title: "Line Chart".into(),
//             content: ViewFn::from(move || {
//                 view! {
//                     <ELineChart
//                         values=chart_values
//                         labels=labels
//                     />
//                 }
//                 .into_any()
//             }),
//             layout: PanelLayout {
//                 x: 20.0,
//                 y: 20.0,
//                 w: 320.0,
//                 h: 220.0,
//             },
//         },
//
//         PanelContent::CalendarHeatmap => Panel {
//             id,
//             title: "Calendar".into(),
//             content: ViewFn::from(move || {
//                 view! {
//                     <ECalendarHeatmap
//                         values=cdata
//                         year=year
//                     />
//                 }
//                 .into_any()
//             }),
//             layout: PanelLayout {
//                 x: 20.0,
//                 y: 20.0,
//                 w: 660.0,
//                 h: 180.0,
//             },
//         },
//     }
// }
