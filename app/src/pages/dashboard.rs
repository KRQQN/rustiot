use crate::{
    components::{
        dashboard::{
            interaction_handler::InteractionHandler, panel::Panel, panel_placer::PanelLayout,
        },
        js_charts::{EBarChart, ECalendarHeatmap, ELineChart},
        side_menu::SideMenu,
    },
    data,
};
use leptos::{html, prelude::*};
use std::collections::HashMap;
use web_sys::MouseEvent;

#[derive(Clone)]
pub struct PanelMeta {
    pub id: String,
    pub title: String,
    pub content: ViewFn,
}

#[component]
pub fn DashboardArea() -> impl IntoView {
    let year = Signal::derive(move || "2026".to_string());
    let cdata = Signal::derive(data::dummy::calendar_heatmap_data);
    let (chart_data, _) = signal(data::dummy::chart_data());
    let (dragging, set_dragging) = signal::<Option<(String, f64, f64)>>(None);
    let (resizing, set_resizing) = signal::<Option<(String, f64, f64, f64, f64)>>(None);
    let canvas_ref = NodeRef::<html::Div>::new();

    const GRID_SIZE: f64 = 20.0;
    const MARGIN: f64 = 10.0;

    let labels = Signal::derive(move || {
        let mut labels = chart_data
            .read()
            .iter()
            .map(|p| p.date.format("%d %b").to_string())
            .collect::<Vec<_>>();

        labels.reverse();
        labels
    });

    let chart_values = Signal::derive(move || {
        let mut values = chart_data
            .read()
            .iter()
            .map(|p| p.value)
            .collect::<Vec<_>>();

        values.reverse();
        values
    });

    //TODO: Look for better solution
    // ========== SEPARATED STATE ==========
    let (metas, _) = signal(vec![
        PanelMeta {
            id: "1".into(),
            title: "Liters".into(),
            content: ViewFn::from(move || {
                view! {<EBarChart values=chart_values labels=labels/>}.into_any()
            }),
        },
        PanelMeta {
            id: "2".into(),
            title: "Temperature".into(),
            content: ViewFn::from(move || {
                view! {<ELineChart values=chart_values labels=labels/>}.into_any()
            }),
        },
        PanelMeta {
            id: "3".into(),
            title: "Usage calendar".into(),
            content: ViewFn::from(move || {
                view! {<ECalendarHeatmap values=cdata year=year/>}.into_any()
            }),
        },
    ]);

    // TODO move to data::dummy::?fn()
    let (layouts, set_layouts) = signal(HashMap::from([
        (
            "1".to_string(),
            PanelLayout {
                x: 20.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        ),
        (
            "2".to_string(),
            PanelLayout {
                x: 360.0,
                y: 20.0,
                w: 320.0,
                h: 220.0,
            },
        ),
        (
            "3".to_string(),
            PanelLayout {
                x: 20.0,
                y: 260.0,
                w: 660.0,
                h: 180.0,
            },
        ),
    ]));

    view! {
        <div class="flex min-h-screen bg-slate-100">
            <SideMenu />
            <div
                node_ref=canvas_ref
                class="flex-1 overflow-auto p-6 relative w-full min-h-screen bg-neutral-900 select-none"
                style=format!(
                    "background-image: \
                        linear-gradient(to right, #333 1px, transparent 1px), \
                        linear-gradient(to bottom, #333 1px, transparent 1px); \
                     background-size: {GRID_SIZE}px {GRID_SIZE}px;"
                )
                on:mousemove=move |ev: MouseEvent| {
                    let Some(canvas) = canvas_ref.get() else { return };
                    let (mx, my) = InteractionHandler::canvas_coords(&ev, &canvas);

                    set_layouts.update(|map| {
                        InteractionHandler::apply_resize(map, &resizing.get(), mx, my);
                        InteractionHandler::apply_move(map, &dragging.get(), mx, my);
                    });
                }
                on:mouseup=move |_| {
                    if let Some((id, ..)) = resizing.get() {
                        set_layouts.update(|map| {
                            InteractionHandler::finish(map, &id, GRID_SIZE, MARGIN, true);
                        });
                        set_resizing.set(None);
                        return;
                    }
                    if let Some((id, ..)) = dragging.get() {
                        set_layouts.update(|map| {
                            InteractionHandler::finish(map, &id, GRID_SIZE, MARGIN, false);
                        });
                        set_dragging.set(None);
                    }
                }
                on:mouseleave=move |_| {
                    if let Some((id, ..)) = resizing.get() {
                        set_layouts.update(|map| {
                            InteractionHandler::finish(map, &id, GRID_SIZE, MARGIN, true);
                        });
                        set_resizing.set(None);
                        return;
                    }
                    if let Some((id, ..)) = dragging.get() {
                        set_layouts.update(|map| {
                            InteractionHandler::finish(map, &id, GRID_SIZE, MARGIN, false);
                        });
                        set_dragging.set(None);
                    }
                }
            >
            <For
                    each=move || metas.get()
                    key=|m| m.id.clone()
                    children=move |meta| {
                        let id = meta.id.clone();
                        let title = meta.title.clone();
                        let content = meta.content.clone();

                        let layout = Signal::derive({
                            let id = id.clone();
                            move || {
                                layouts.with(|map| {
                                    map.get(&id).cloned().unwrap_or(PanelLayout {
                                        x: 0.0, y: 0.0, w: 200.0, h: 150.0,
                                    })
                                })
                            }
                        });

                        let is_dragging = Signal::derive({
                            let id = id.clone();
                            move || {
                                dragging.get().map(|(did, ..)| did == id).unwrap_or(false)
                            }
                        });

                        view! {
                            <Panel
                                id=id
                                title=title
                                layout=layout
                                is_dragging=is_dragging
                                set_dragging=set_dragging
                                set_resizing=set_resizing
                                canvas_ref=canvas_ref
                                content=content
                            />
                        }
                    }
                />
                </div>
        </div>
    }
}
