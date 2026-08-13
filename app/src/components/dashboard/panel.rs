use crate::components::dashboard::{
    interaction_handler::InteractionHandler, panel_placer::PanelLayout,
};
use leptos::{html, prelude::*};
use web_sys::MouseEvent;

#[derive(Clone)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub content: ViewFn,
    pub layout: PanelLayout,
}

#[derive(Clone)]
pub enum PanelInteraction {
    Dragging {
        id: String,
        offset_x: f64,
        offset_y: f64,
    },
    Resizing {
        id: String,
        start_mx: f64,
        start_my: f64,
        start_w: f64,
        start_h: f64,
    },
    Removing {
        id: String,
    },
}

#[component]
pub fn Panel(
    id: String,
    title: String,
    layout: Signal<PanelLayout>,
    interaction: ReadSignal<Option<PanelInteraction>>,
    set_interaction: WriteSignal<Option<PanelInteraction>>,
    canvas_ref: NodeRef<html::Div>,
    content: ViewFn,
) -> impl IntoView {
    let id_for_drag = id.clone();
    let id_for_resize = id.clone();
    let id_for_remove = id.clone();
    let (menu_open, set_menu_open) = signal(false);

    let is_this_dragging = Memo::new({
        let id = id.clone();
        move |_| {
            matches!(
                interaction.get(),
                Some(PanelInteraction::Dragging { id: ref did, .. }) if *did == id
            )
        }
    });

    view! {
        <div
            class=move || {
                let base = "absolute bg-neutral-800 border border-neutral-600 rounded-lg p-3 cursor-grab shadow-lg group";
                if is_this_dragging.get() {
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

                let Some(canvas) = canvas_ref.get() else {
                    return;
                };

                let (mx, my) = InteractionHandler::canvas_coords(&ev, &canvas);
                let l = layout.get();

                set_interaction.set(Some(PanelInteraction::Dragging {
                    id: id_for_drag.clone(),
                    offset_x: mx - l.x,
                    offset_y: my - l.y,
                }));
            }
        >
            // Header
            <div class="flex items-center justify-between mb-2 relative">
                <div class="font-medium text-sm text-neutral-200 pointer-events-none truncate pr-2">
                    {title}
                </div>

                <div class="relative shrink-0">
                    <button
                        class="w-6 h-6 flex items-center justify-center rounded
                               text-neutral-500 hover:text-neutral-200 hover:bg-neutral-700
                               opacity-0 group-hover:opacity-100 transition-all"
                        title="Panel options"
                        on:mousedown=move |ev: MouseEvent| {
                            if ev.button() != 0 {
                                return;
                            }
                            ev.prevent_default();
                            ev.stop_propagation();
                            set_menu_open.update(|v| *v = !*v);
                        }
                    >
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <circle cx="12" cy="5" r="1.5"/>
                            <circle cx="12" cy="12" r="1.5"/>
                            <circle cx="12" cy="19" r="1.5"/>
                        </svg>
                    </button>

                    <Show when=move || menu_open.get()>
                        <div
                            class="absolute right-0 top-full mt-1 w-36
                                   bg-neutral-900 border border-neutral-700
                                   rounded-lg shadow-xl overflow-hidden z-50"
                            on:mousedown=move |ev: MouseEvent| {
                                ev.stop_propagation();
                            }
                        >
                            <button
                                class="w-full flex items-center gap-2 px-3 py-2 text-sm
                                       text-neutral-300 hover:bg-neutral-800 hover:text-white
                                       transition-colors text-left"
                            on:mousedown={
                                let id = id_for_remove.clone();
                                move |ev: MouseEvent| {
                                    if ev.button() != 0 {
                                        return;
                                    }
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    set_menu_open.set(false);

                                    set_interaction.set(Some(PanelInteraction::Removing {
                                        id: id.clone(),
                                    }));
                                }
                            }>
                                <svg class="w-3.5 h-3.5 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                </svg>
                                "Edit"
                            </button>

                            <div class="border-t border-neutral-800"></div>

                            <button
                                class="w-full flex items-center gap-2 px-3 py-2 text-sm
                                       text-red-400 hover:bg-red-500/10 hover:text-red-300
                                       transition-colors text-left"
                            on:mousedown={
                                let id = id_for_remove.clone();
                                move |ev: MouseEvent| {
                                    if ev.button() != 0 {
                                        return;
                                    }
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    set_menu_open.set(false);

                                    set_interaction.set(Some(PanelInteraction::Removing {
                                        id: id.clone(),
                                    }));
                                }
                            }
                        >
                                <svg class="w-3.5 h-3.5" fill="nlet (menu_open, set_menu_open) = signal(false);let (menu_open, set_menu_open) = signal(false);one" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                          d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                </svg>
                                "Remove"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>

            // Content
            <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded overflow-hidden">
                {content.run()}
            </div>

            // Resize handle
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

                    let Some(canvas) = canvas_ref.get() else {
                        return;
                    };

                    let (mx, my) = InteractionHandler::canvas_coords(&ev, &canvas);
                    let l = layout.get();

                    set_interaction.set(Some(PanelInteraction::Resizing {
                        id: id_for_resize.clone(),
                        start_mx: mx,
                        start_my: my,
                        start_w: l.w,
                        start_h: l.h,
                    }));
                }
            >
            </div>
        </div>
    }
}
