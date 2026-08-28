use crate::{
    components::{
        dashboard::{
            interaction_handler::InteractionHandler,
            panel::{Panel, PanelConfig, PanelInteraction},
            panel_placer::{PanelLayout, PanelPlacer},
            toolbar::PanelToolbar,
        },
        side_menu::SideMenu,
        // toast::InfluxToast,
    },
    data::{self, dummy::db_read_get_panels},
};
use leptos::{html, prelude::*};
use web_sys::MouseEvent;

#[component]
pub fn DashboardArea() -> impl IntoView {
    let canvas_ref = NodeRef::<html::Div>::new();
    let (interaction, set_interaction) = signal::<Option<PanelInteraction>>(None);
    let (panels, set_panels) = signal::<Vec<Panel>>(vec![]);

    const GRID_SIZE: f64 = 20.0;
    const MARGIN: f64 = 10.0;

    #[cfg(not(feature = "ssr"))]
    leptos::task::spawn_local(async move {
        let loaded_panels = db_read_get_panels().await;
        set_panels.set(loaded_panels);
    });

    let add_panel = Callback::new(move |cfg: PanelConfig| {
        set_panels.update(|panels| {
            let id = format!("panel-{}", panels.len() + 1);

            let mut panel = data::dummy::new_panel(cfg, id);
            // TODO Save user panels, read user panels

            let others: Vec<&PanelLayout> = panels.iter().map(|p| &p.layout).collect();
            let (x, y) = PanelPlacer::find_nearest_free(&panel.layout, &others, GRID_SIZE, MARGIN);
            panel.layout.x = x;
            panel.layout.y = y;
            panels.push(panel);
        });
    });
    let finish_current = move || {
        let Some(interaction) = interaction.get() else {
            return;
        };

        match &interaction {
            PanelInteraction::Removing { id } => {
                set_panels.update(|panels| {
                    panels.retain(|p| p.id != *id);
                });
            }
            _ => {
                set_panels.update(|panels| {
                    InteractionHandler::finish(panels, &interaction, GRID_SIZE, MARGIN);
                });
            }
        }

        set_interaction.set(None);
    };

    view! {
        <div class="flex min-h-screen bg-slate-100">
            // <InfluxToast/>
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
