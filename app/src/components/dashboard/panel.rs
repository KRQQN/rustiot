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

    let is_this_dragging = Memo::new({
        let id = id.clone();
        move |_| {
            matches!(
                interaction.get(),
                Some(PanelInteraction::Dragging {
                    id: ref did,
                    offset_x: _,
                    offset_y: _,
                }) if *did == id
            )
        }
    });

    view! {
        <div
            class=move || {
                let base = "absolute bg-neutral-800 border border-neutral-600 rounded-lg p-3 cursor-grab shadow-lg";
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
            ></div>

            <div class="font-medium text-sm text-neutral-200 mb-2 pointer-events-none">
                {title}
            </div>

            <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded overflow-hidden">
                {content.run()}
            </div>
        </div>
    }
}
