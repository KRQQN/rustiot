use leptos::prelude::*;

use crate::{
    components::{
        dashboard::panel::PanelContent,
        js_charts::{EBarChart, ELineChart},
    },
    server_fns::get_device_data::get_device_data,
};

#[component]
pub fn Chart(device_id: String, timespan: String, chart_type: PanelContent) -> impl IntoView {
    let readings = Resource::new(
        {
            let device_id = device_id.clone();
            let timespan = timespan.clone();
            move || (device_id.clone(), timespan.clone())
        },
        |(id, ts)| async move { get_device_data(id, ts, Some(100)).await },
    );

    let points = Signal::derive(move || {
        readings
            .get()
            .and_then(|r| r.ok())
            .map(|d| {
                d.iter()
                    .map(|r| {
                        // [time_string, temperature]
                        serde_json::json!([r.time, r.temperature])
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });

    view! {
        <Suspense fallback=move || view! { <p class="text-neutral-500 text-sm p-4">"Loading…"</p> }>
            {move || {
                match chart_type {
                PanelContent::LineChart => {
                    view! { <ELineChart  points=points/>}.into_any()
                }
                PanelContent::BarChart => {
                    view! { <EBarChart points=points/>}.into_any()
                }
                PanelContent::CalendarHeatmap => {view!{}.into_any()}
            }}}
        </Suspense>
    }
}
