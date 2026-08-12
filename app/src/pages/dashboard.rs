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
use web_sys::MouseEvent;

#[component]
pub fn DashboardArea() -> impl IntoView {
    let canvas_ref = NodeRef::<html::Div>::new();
    let (dragging, set_dragging) = signal::<Option<(String, f64, f64)>>(None);
    let (resizing, set_resizing) = signal::<Option<(String, f64, f64, f64, f64)>>(None);

    const GRID_SIZE: f64 = 20.0;
    const MARGIN: f64 = 10.0;

    // ──── Temporary data/signals ────────────────────
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
    // ─────────────────────────────────────────────────

    let (panels, set_panels) = signal(vec![
        Panel {
            id: "1".into(),
            title: "Liters".into(),
            content: ViewFn::from(move || {
                view! {<EBarChart values=chart_values labels=labels/>}.into_any()
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
                view! {<ELineChart values=chart_values labels=labels/>}.into_any()
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
                view! {<ECalendarHeatmap values=cdata year=year/>}.into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 260.0,
                w: 660.0,
                h: 180.0,
            },
        },
    ]);

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

                    set_panels.update(|panels| {
                        if let Some((id, ..)) = resizing.get() {
                            if let Some(panel) = panels.iter_mut().find(|p| p.id == id) {
                                InteractionHandler::apply_resize(
                                    &mut panel.layout,
                                    &resizing.get(),
                                    mx,
                                    my,
                                );
                            }
                        }

                        if let Some((id, ..)) = dragging.get() {
                            if let Some(panel) = panels.iter_mut().find(|p| p.id == id) {
                                InteractionHandler::apply_move(
                                    &mut panel.layout,
                                    &dragging.get(),
                                    mx,
                                    my,
                                );
                            }
                        }
                    });
                }
                on:mouseup=move |_| {
                    if let Some((id, ..)) = resizing.get() {
                        set_panels.update(|panels| {
                            InteractionHandler::finish(panels, &id, GRID_SIZE, MARGIN, true);
                        });
                        set_resizing.set(None);
                        return;
                    }

                    if let Some((id, ..)) = dragging.get() {
                        set_panels.update(|panels| {
                            InteractionHandler::finish(panels, &id, GRID_SIZE, MARGIN, false);
                        });
                        set_dragging.set(None);
                    }
                }
                on:mouseleave=move |_| {
                    if let Some((id, ..)) = resizing.get() {
                        set_panels.update(|panels| {
                            InteractionHandler::finish(panels, &id, GRID_SIZE, MARGIN, true);
                        });
                        set_resizing.set(None);
                        return;
                    }

                    if let Some((id, ..)) = dragging.get() {
                        set_panels.update(|panels| {
                            InteractionHandler::finish(panels, &id, GRID_SIZE, MARGIN, false);
                        });
                        set_dragging.set(None);
                    }
                }
            >
                <For
                    each=move || panels.get()
                    key=|p| p.id.clone()
                    children=move |panel| {
                        let id = panel.id.clone();
                        let title = panel.title.clone();
                        let content = panel.content.clone();

                        let layout = Memo::new({
                            let id = id.clone();
                            move |_| {
                                panels.with(|ps| {
                                    ps.iter()
                                        .find(|p| p.id == id)
                                        .map(|p| p.layout.clone())
                                        .unwrap_or(PanelLayout {
                                            x: 0.0,
                                            y: 0.0,
                                            w: 200.0,
                                            h: 150.0,
                                        })
                                })
                            }
                        });

                        let is_dragging = Memo::new({
                            let id = id.clone();
                            move |_| {
                                dragging
                                    .get()
                                    .map(|(did, ..)| did == id)
                                    .unwrap_or(false)
                            }
                        });

                        view! {
                            <Panel
                                id=id
                                title=title
                                layout=layout.into()
                                is_dragging=is_dragging.into()
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
