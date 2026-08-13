use crate::{
    components::{
        dashboard::{
            interaction_handler::InteractionHandler,
            panel::{Panel, PanelInteraction},
            panel_placer::{PanelLayout, PanelPlacer},
            toolbar::PanelToolbar,
        },
        js_charts::{EBarChart, ECalendarHeatmap, ELineChart},
        side_menu::SideMenu,
    },
    data,
};
use leptos::{html, prelude::*};
use web_sys::MouseEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum PanelType {
    BarChart,
    LineChart,
    CalendarHeatmap,
}

#[component]
pub fn DashboardArea() -> impl IntoView {
    let canvas_ref = NodeRef::<html::Div>::new();

    let (interaction, set_interaction) = signal::<Option<PanelInteraction>>(None);

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
                view! { <EBarChart values=chart_values labels=labels/> }.into_any()
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
                view! { <ELineChart values=chart_values labels=labels/> }.into_any()
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
                view! { <ECalendarHeatmap values=cdata year=year/> }.into_any()
            }),
            layout: PanelLayout {
                x: 20.0,
                y: 260.0,
                w: 660.0,
                h: 180.0,
            },
        },
    ]);

    let add_panel = Callback::new(move |panel_type: PanelType| {
        set_panels.update(|panels| {
            let id = format!("panel-{}", panels.len() + 1);
            let (title, content) = match panel_type {
                PanelType::BarChart => (
                    "Bar Chart".into(),
                    ViewFn::from(move || {
                        view! { <EBarChart values=chart_values labels=labels/> }.into_any()
                    }),
                ),
                PanelType::LineChart => (
                    "Line Chart".into(),
                    ViewFn::from(move || {
                        view! { <ELineChart values=chart_values labels=labels/> }.into_any()
                    }),
                ),
                PanelType::CalendarHeatmap => (
                    "Calendar".into(),
                    ViewFn::from(move || {
                        view! { <ECalendarHeatmap values=cdata year=year/> }.into_any()
                    }),
                ),
            };

            // Needs a better solution for this, also used in interaction_handler.rs
            let others: Vec<&PanelLayout> = panels
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != panels.len() + 1)
                .map(|(_, p)| &p.layout)
                .collect();

            let mut layout = PanelLayout {
                x: 40.0,
                y: 40.0,
                w: 320.0,
                h: 220.0,
            };

            let (x, y) = PanelPlacer::find_nearest_free(&layout, &others, GRID_SIZE, MARGIN);
            layout.x = x;
            layout.y = y;

            panels.push(Panel {
                id,
                title,
                content,
                layout,
            });
        });
    });

    let finish_current = move || {
        let Some(interaction) = interaction.get() else {
            return;
        };

        set_panels.update(|panels| {
            InteractionHandler::finish(panels, &interaction, GRID_SIZE, MARGIN);
        });

        set_interaction.set(None);
    };

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
                    let Some(canvas) = canvas_ref.get() else {
                        return;
                    };

                    let (mx, my) =
                        InteractionHandler::canvas_coords(&ev, &canvas);

                    let Some(interaction) = interaction.get() else {
                        return;
                    };

                    set_panels.update(|panels| {
                        InteractionHandler::apply(
                            panels,
                            &interaction,
                            mx,
                            my,
                        );
                    });
                }
                on:mouseup=move |_| {
                    finish_current();
                }
                on:mouseleave=move |_| {
                    finish_current();
                }
            >
                <PanelToolbar on_add=add_panel />
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

                        view! {
                            <Panel
                                id=id
                                title=title
                                layout=layout.into()
                                interaction=interaction
                                set_interaction=set_interaction
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
