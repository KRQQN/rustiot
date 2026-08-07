use crate::{
    components::{
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

#[derive(Clone, Debug, PartialEq)]
pub struct PanelLayout {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

const GRID_SIZE: f64 = 20.0;
const MARGIN: f64 = 10.0;

// TODO: Separate out helpers
fn snap(value: f64) -> f64 {
    (value / GRID_SIZE).round() * GRID_SIZE
}

fn overlaps(a: &PanelLayout, b: &PanelLayout) -> bool {
    let ax2 = a.x + a.w + MARGIN;
    let ay2 = a.y + a.h + MARGIN;
    let bx2 = b.x + b.w + MARGIN;
    let by2 = b.y + b.h + MARGIN;
    a.x < bx2 && ax2 > b.x && a.y < by2 && ay2 > b.y
}

fn find_nearest_free(
    id: &str,
    layout: &PanelLayout,
    layouts: &HashMap<String, PanelLayout>,
) -> (f64, f64) {
    let others: Vec<&PanelLayout> = layouts
        .iter()
        .filter(|(other_id, _)| *other_id != id)
        .map(|(_, l)| l)
        .collect();

    if others.iter().all(|o| !overlaps(layout, o)) {
        return (layout.x, layout.y);
    }

    let max_steps = 40;
    for step in 1..=max_steps {
        let dist = step as f64 * GRID_SIZE;

        let candidates = [
            (layout.x + dist, layout.y),
            (layout.x - dist, layout.y),
            (layout.x, layout.y + dist),
            (layout.x, layout.y - dist),
            (layout.x + dist, layout.y + dist),
            (layout.x + dist, layout.y - dist),
            (layout.x - dist, layout.y + dist),
            (layout.x - dist, layout.y - dist),
        ];

        for (cx, cy) in candidates {
            let test = PanelLayout {
                x: snap(cx.max(0.0)),
                y: snap(cy.max(0.0)),
                w: layout.w,
                h: layout.h,
            };
            if others.iter().all(|o| !overlaps(&test, o)) {
                return (test.x, test.y);
            }
        }
    }

    (snap(layout.x.max(0.0)), snap(layout.y.max(0.0)))
}

#[component]
pub fn DashboardArea() -> impl IntoView {
    let year = Signal::derive(move || "2026".to_string());

    let cdata = Signal::derive(data::dummy::calendar_heatmap_data);

    let (chart_data, _) = signal(data::dummy::chart_data());

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
                // TODO: Separate event-handlers to an e.g "InteractionHandler"
                on:mousemove=move |ev: MouseEvent| {
                    let Some(canvas) = canvas_ref.get() else { return; };
                    let rect = canvas.get_bounding_client_rect();
                    let mouse_x = f64::from(ev.client_x()) - rect.left();
                    let mouse_y = f64::from(ev.client_y()) - rect.top();

                    if let Some((id, start_mx, start_my, start_w, start_h)) = resizing.get() {
                        let dx = mouse_x - start_mx;
                        let dy = mouse_y - start_my;
                        let new_w = (start_w + dx).max(120.0);
                        let new_h = (start_h + dy).max(80.0);

                        set_layouts.update(|map| {
                            if let Some(l) = map.get_mut(&id) {
                                l.w = new_w;
                                l.h = new_h;
                            }
                        });
                        return;
                    }

                    let Some((id, offset_x, offset_y)) = dragging.get() else { return; };

                    let new_x = (mouse_x - offset_x).max(0.0);
                    let new_y = (mouse_y - offset_y).max(0.0);

                    set_layouts.update(|map| {
                        if let Some(l) = map.get_mut(&id) {
                            l.x = new_x;
                            l.y = new_y;
                        }
                    });
                }
                on:mouseup=move |_| {
                    if let Some((id, _, _, _, _)) = resizing.get() {
                        set_layouts.update(|map| {
                            if let Some(l) = map.get_mut(&id) {
                                l.w = snap(l.w);
                                l.h = snap(l.h);
                            }
                        });

                        set_layouts.update(|map| {
                            if let Some(layout) = map.get(&id).cloned() {
                                let (fx, fy) = find_nearest_free(&id, &layout, map);
                                if let Some(l) = map.get_mut(&id) {
                                    l.x = fx;
                                    l.y = fy;
                                }
                            }
                        });

                        set_resizing.set(None);
                        return;
                    }

                    let Some((id, _, _)) = dragging.get() else { return; };

                    set_layouts.update(|map| {
                        if let Some(layout) = map.get(&id).cloned() {
                            let (fx, fy) = find_nearest_free(&id, &layout, map);
                            if let Some(l) = map.get_mut(&id) {
                                l.x = fx;
                                l.y = fy;
                            }
                        }
                    });

                    set_dragging.set(None);
                }
                on:mouseleave=move |_| {
                    if let Some((id, _, _, _, _)) = resizing.get() {
                        set_layouts.update(|map| {
                            if let Some(l) = map.get_mut(&id) {
                                l.w = snap(l.w);
                                l.h = snap(l.h);
                            }
                        });

                        set_layouts.update(|map| {
                            if let Some(layout) = map.get(&id).cloned() {
                                let (fx, fy) = find_nearest_free(&id, &layout, map);
                                if let Some(l) = map.get_mut(&id) {
                                    l.x = fx;
                                    l.y = fy;
                                }
                            }
                        });

                        set_resizing.set(None);
                        return;
                    }

                    let Some((id, _, _)) = dragging.get() else { return; };

                    set_layouts.update(|map| {
                        if let Some(layout) = map.get(&id).cloned() {
                            let (fx, fy) = find_nearest_free(&id, &layout, map);
                            if let Some(l) = map.get_mut(&id) {
                                l.x = fx;
                                l.y = fy;
                            }
                        }
                    });

                    set_dragging.set(None);
                }
            >
                // TODO: Obvious separate component
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
                                    format!("left: {}px; top: {}px; width: {}px; height: {}px;", l.x, l.y, l.w, l.h)
                                }
                                on:mousedown=move |ev: MouseEvent| {
                                    if ev.button() != 0 { return; }
                                    ev.prevent_default();
                                    ev.stop_propagation();

                                    let Some(canvas) = canvas_ref.get() else { return; };
                                    let rect = canvas.get_bounding_client_rect();
                                    let l = layout.get();

                                    let mouse_x = f64::from(ev.client_x()) - rect.left();
                                    let mouse_y = f64::from(ev.client_y()) - rect.top();

                                    set_dragging.set(Some((
                                        id_for_drag.clone(),
                                        mouse_x - l.x,
                                        mouse_y - l.y,
                                    )));
                                }
                            >
                                <div
                                    class="absolute bottom-0 right-0 w-4 h-4 cursor-nwse-resize
                                           bg-neutral-500 hover:bg-blue-500 rounded-tl
                                           opacity-60 hover:opacity-100 transition-opacity z-20"
                                    on:mousedown=move |ev: MouseEvent| {
                                        if ev.button() != 0 { return; }
                                        ev.prevent_default();
                                        ev.stop_propagation();

                                        let Some(canvas) = canvas_ref.get() else { return; };
                                        let rect = canvas.get_bounding_client_rect();
                                        let l = layout.get();

                                        let mouse_x = f64::from(ev.client_x()) - rect.left();
                                        let mouse_y = f64::from(ev.client_y()) - rect.top();

                                        set_resizing.set(Some((
                                            id_for_resize.clone(),
                                            mouse_x,
                                            mouse_y,
                                            l.w,
                                            l.h,
                                        )));
                                    }
                                ></div>

                                <div class="font-medium text-sm text-neutral-200 mb-2 pointer-events-none">
                                    {title.clone()}
                                </div>

                                <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded overflow-hidden">
                                    {match content {
                                        PanelContent::BarChart => {
                                            view! { <EBarChart values=chart_values labels=labels /> }.into_any()
                                        }
                                        PanelContent::LineChart => {
                                            view! { <ELineChart values=chart_values labels=labels /> }.into_any()
                                        }
                                        PanelContent::CalendarHeatmap => {
                                            view! { <ECalendarHeatmap values=cdata year=year /> }.into_any()
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
