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

#[component]
pub fn Panel(
    id: String,
    title: String,
    layout: Signal<PanelLayout>,
    is_dragging: Signal<bool>,
    set_dragging: WriteSignal<Option<(String, f64, f64)>>,
    set_resizing: WriteSignal<Option<(String, f64, f64, f64, f64)>>,
    canvas_ref: NodeRef<html::Div>,
    content: ViewFn,
) -> impl IntoView {
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
                {title}
            </div>

            <div class="h-[calc(100%-2rem)] bg-neutral-700/40 rounded overflow-hidden">
                {content.run()}
            </div>
        </div>
    }
}

pub trait AsPanel {
    fn id(&self) -> &str;
    fn layout(&self) -> &PanelLayout;
    fn layout_mut(&mut self) -> &mut PanelLayout;
}

impl AsPanel for Panel {
    fn id(&self) -> &str {
        &self.id
    }
    fn layout(&self) -> &PanelLayout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut PanelLayout {
        &mut self.layout
    }
}
