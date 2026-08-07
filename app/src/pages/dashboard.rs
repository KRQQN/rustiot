use crate::{
    components::{
        dashboard::{
            interaction_handler::InteractionHandler,
            panel_placer::{PanelLayout, PanelPlacer},
        },
        js_charts::{EBarChart, ECalendarHeatmap, ELineChart},
        side_menu::SideMenu,
    },
    data,
};
use leptos::{html, prelude::*};
use std::collections::HashMap;
use web_sys::MouseEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum PanelContent {
    BarChart,
    LineChart,
    CalendarHeatmap,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanelMeta {
    pub id: String,
    pub title: String,
    pub content: PanelContent,
}

#[component]
pub fn DashboardArea() -> impl IntoView {
    let year = Signal::derive(move || "2026".to_string());
    let cdata = Signal::derive(data::dummy::calendar_heatmap_data);
    let (chart_data, _) = signal(data::dummy::chart_data());

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
            title: "CPU".into(),
            content: PanelContent::BarChart,
        },
        PanelMeta {
            id: "2".into(),
            title: "Memory".into(),
            content: PanelContent::LineChart,
        },
        PanelMeta {
            id: "3".into(),
            title: "Network".into(),
            content: PanelContent::CalendarHeatmap,
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

    let (dragging, set_dragging) = signal::<Option<(String, f64, f64)>>(None);
    let (resizing, set_resizing) = signal::<Option<(String, f64, f64, f64, f64)>>(None);
    let canvas_ref = NodeRef::<html::Div>::new();

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
                                        x: 0.0,
                                        y: 0.0,
                                        w: 200.0,
                                        h: 150.0,
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

                        let id_for_drag = id.clone();
                        let id_for_resize = id.clone();

                        view! {
                            <div
                                class=move || {
                                    let base = "absolute bg-neutral-800 border border-neutral-600 rounded-lg p-3 cursor-grab shadow-lg";
                                    if is_dragging.get() {
                                        format!("{base} cursor-grabbing z-50 ring-2 ring-blue-500 opacity-90")
                                    } else {
                                        format!("{base} z-10")
                                    }
                                }
                                style=move || {
                                    let l = layout.get();
                                    format!(
                                        "left: {}px; top: {}px; width: {}px; height: {}px;",
                                        l.x, l.y, l.w, l.h
                                    )
                                }
                                on:mousedown=move |ev: MouseEvent| {
                                    if ev.button() != 0 {
                                        return;
                                    }
                                    ev.prevent_default();
                                    ev.stop_propagation();

                                    let Some(canvas) = canvas_ref.get() else { return };
                                    let (mx, my) = InteractionHandler::canvas_coords(&ev, &canvas);
                                    let l = layout.get();

                                    set_dragging.set(
                                        InteractionHandler::start_drag(id_for_drag.clone(), &l, mx, my)
                                    );
                                }
                            >
                                <div
                                    class="absolute bottom-0 right-0 w-4 h-4 cursor-nwse-resize
                                       bg-neutral-500 hover:bg-blue-500 rounded-tl
                                       opacity-60 hover:opacity-100 transition-opacity z-20"
                                    on:mousedown=move |ev: MouseEvent| {
                                        if ev.button() != 0 {
                                            return;
                                        }
                                        ev.prevent_default();
                                        ev.stop_propagation();

                                        let Some(canvas) = canvas_ref.get() else { return };
                                        let (mx, my) = InteractionHandler::canvas_coords(&ev, &canvas);
                                        let l = layout.get();

                                        set_resizing.set(
                                            InteractionHandler::start_resize(
                                                id_for_resize.clone(),
                                                &l,
                                                mx,
                                                my,
                                            )
                                        );
                                    }
                                ></div>

                                <div class="font-medium text-sm text-neutral-200 mb-2 pointer-events-none">
                                    {title.clone()}
                                </div>

                                <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded overflow-hidden">
                                    {match content {
                                        PanelContent::BarChart => {
                                            view! {
                                                <EBarChart values=chart_values labels=labels />
                                            }
                                            .into_any()
                                        }
                                        PanelContent::LineChart => {
                                            view! {
                                                <ELineChart values=chart_values labels=labels />
                                            }
                                            .into_any()
                                        }
                                        PanelContent::CalendarHeatmap => {
                                            view! {
                                                <ECalendarHeatmap values=cdata year=year />
                                            }
                                            .into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
