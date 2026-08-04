use leptos::{html, prelude::*};
use web_sys::MouseEvent;

use crate::components::side_menu::SideMenu;

#[derive(Clone, Debug, PartialEq)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

// ===== Settings =====
const GRID_SIZE: f64 = 20.0;
const MARGIN: f64 = 10.0;

fn snap(value: f64) -> f64 {
    (value / GRID_SIZE).round() * GRID_SIZE
}

fn overlaps(a: &Panel, b: &Panel) -> bool {
    let ax2 = a.x + a.w + MARGIN;
    let ay2 = a.y + a.h + MARGIN;
    let bx2 = b.x + b.w + MARGIN;
    let by2 = b.y + b.h + MARGIN;

    a.x < bx2 && ax2 > b.x && a.y < by2 && ay2 > b.y
}

/// Find the nearest non-overlapping position (simple spiral search)
fn find_nearest_free(panel: &Panel, others: &[Panel]) -> (f64, f64) {
    if others.iter().all(|o| !overlaps(panel, o)) {
        return (panel.x, panel.y);
    }

    // Spiral search outward from the dropped position
    let max_steps = 40;
    for step in 1..=max_steps {
        let dist = step as f64 * GRID_SIZE;

        // Try 8 directions
        let candidates = [
            (panel.x + dist, panel.y),
            (panel.x - dist, panel.y),
            (panel.x, panel.y + dist),
            (panel.x, panel.y - dist),
            (panel.x + dist, panel.y + dist),
            (panel.x + dist, panel.y - dist),
            (panel.x - dist, panel.y + dist),
            (panel.x - dist, panel.y - dist),
        ];

        for (cx, cy) in candidates {
            let mut test = panel.clone();
            test.x = snap(cx.max(0.0));
            test.y = snap(cy.max(0.0));

            if others.iter().all(|o| !overlaps(&test, o)) {
                return (test.x, test.y);
            }
        }
    }

    // Fallback – just put it at the snapped drop position
    (snap(panel.x.max(0.0)), snap(panel.y.max(0.0)))
}

#[component]
pub fn DashboardArea() -> impl IntoView {
    let (panels, set_panels) = signal(vec![
        Panel {
            id: "1".into(),
            title: "CPU".into(),
            x: 20.0,
            y: 20.0,
            w: 320.0,
            h: 220.0,
        },
        Panel {
            id: "2".into(),
            title: "Memory".into(),
            x: 360.0,
            y: 20.0,
            w: 320.0,
            h: 220.0,
        },
        Panel {
            id: "3".into(),
            title: "Network".into(),
            x: 20.0,
            y: 260.0,
            w: 660.0,
            h: 180.0,
        },
    ]);

    // (id, grab_offset_x, grab_offset_y)
    let (dragging, set_dragging) = signal::<Option<(String, f64, f64)>>(None);

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
                let Some((id, offset_x, offset_y)) = dragging.get() else {
                    return;
                };

                let Some(canvas) = canvas_ref.get() else {
                    return;
                };
                let rect = canvas.get_bounding_client_rect();

                let mouse_x = f64::from(ev.client_x()) - rect.left();
                let mouse_y = f64::from(ev.client_y()) - rect.top();

                // Free movement while dragging (no collision check)
                let new_x = (mouse_x - offset_x).max(0.0);
                let new_y = (mouse_y - offset_y).max(0.0);

                set_panels.update(|list| {
                    if let Some(p) = list.iter_mut().find(|p| p.id == id) {
                        p.x = new_x;
                        p.y = new_y;
                    }
                });
            }
            on:mouseup=move |_| {
                let Some((id, _, _)) = dragging.get() else {
                    return;
                };

                // Resolve overlaps only on drop
                set_panels.update(|list| {
                    let Some(idx) = list.iter().position(|p| p.id == id) else {
                        return;
                    };

                    let panel = list[idx].clone();
                    let others: Vec<Panel> = list
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|(_, p)| p.clone())
                        .collect();

                    let (free_x, free_y) = find_nearest_free(&panel, &others);

                    list[idx].x = free_x;
                    list[idx].y = free_y;
                });

                set_dragging.set(None);
            }
            on:mouseleave=move |_| {
                // Treat leaving the canvas the same as mouseup
                let Some((id, _, _)) = dragging.get() else {
                    return;
                };

                set_panels.update(|list| {
                    let Some(idx) = list.iter().position(|p| p.id == id) else {
                        return;
                    };

                    let panel = list[idx].clone();
                    let others: Vec<Panel> = list
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|(_, p)| p.clone())
                        .collect();

                    let (free_x, free_y) = find_nearest_free(&panel, &others);

                    list[idx].x = free_x;
                    list[idx].y = free_y;
                });

                set_dragging.set(None);
            }
        >
            <For
                each=move || panels.get()
                key=|p| p.id.clone()
                children=move |panel_snapshot| {
                    let id = panel_snapshot.id.clone();

                    let panel = Signal::derive({
                        let id = id.clone();
                        move || {
                            panels
                                .get()
                                .into_iter()
                                .find(|p| p.id == id)
                                .expect("panel must exist")
                        }
                    });

                    let is_dragging = Signal::derive({
                        let id = id.clone();
                        move || {
                            dragging
                                .get()
                                .map(|(did, ..)| did == id)
                                .unwrap_or(false)
                        }
                    });

                    view! {
                        <div
                            class=move || {
                                let base = "absolute bg-neutral-800 border border-neutral-600 \
                                            rounded-lg p-3 cursor-grab shadow-lg";
                                if is_dragging.get() {
                                    format!("{base} cursor-grabbing z-50 ring-2 ring-blue-500 opacity-90")
                                } else {
                                    format!("{base} z-10")
                                }
                            }
                            style=move || {
                                let p = panel.get();
                                format!(
                                    "left: {}px; top: {}px; width: {}px; height: {}px;",
                                    p.x, p.y, p.w, p.h
                                )
                            }
                            on:mousedown=move |ev: MouseEvent| {
                                if ev.button() != 0 {
                                    return;
                                }
                                ev.prevent_default();
                                ev.stop_propagation();

                                let Some(canvas) = canvas_ref.get() else {
                                    return;
                                };
                                let rect = canvas.get_bounding_client_rect();
                                let p = panel.get();

                                let mouse_x = f64::from(ev.client_x()) - rect.left();
                                let mouse_y = f64::from(ev.client_y()) - rect.top();

                                let offset_x = mouse_x - p.x;
                                let offset_y = mouse_y - p.y;

                                set_dragging.set(Some((id.clone(), offset_x, offset_y)));
                            }
                        >
                            <div class="font-medium text-sm text-neutral-200 mb-2 pointer-events-none">
                                {move || panel.get().title.clone()}
                            </div>

                            <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded flex items-center justify-center text-neutral-400 text-sm pointer-events-none">
                                "Panel content"
                            </div>
                        </div>
                    }
                }
            />
        </div>
                </div>
    }
}
