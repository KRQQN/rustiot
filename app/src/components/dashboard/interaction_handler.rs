use std::collections::HashMap;
use web_sys::MouseEvent;

use crate::components::dashboard::panel_placer::{PanelLayout, PanelPlacer};

pub type DragState = Option<(String, f64, f64)>; // (id, offset_x, offset_y)
pub type ResizeState = Option<(String, f64, f64, f64, f64)>; // (id, start_mx, start_my, start_w, start_h)

pub struct InteractionHandler;

impl InteractionHandler {
    /// Convert client coordinates → canvas-local coordinates
    pub fn canvas_coords(ev: &MouseEvent, canvas: &web_sys::HtmlElement) -> (f64, f64) {
        let rect = canvas.get_bounding_client_rect();
        (
            f64::from(ev.client_x()) - rect.left(),
            f64::from(ev.client_y()) - rect.top(),
        )
    }

    // ── Start actions ──────────────────────────────────────────────

    pub fn start_drag(id: String, layout: &PanelLayout, mouse_x: f64, mouse_y: f64) -> DragState {
        Some((id, mouse_x - layout.x, mouse_y - layout.y))
    }

    pub fn start_resize(
        id: String,
        layout: &PanelLayout,
        mouse_x: f64,
        mouse_y: f64,
    ) -> ResizeState {
        Some((id, mouse_x, mouse_y, layout.w, layout.h))
    }

    // ── Continuous update (mousemove) ──────────────────────────────

    pub fn apply_move(
        layouts: &mut HashMap<String, PanelLayout>,
        dragging: &DragState,
        mouse_x: f64,
        mouse_y: f64,
    ) {
        let Some((id, offset_x, offset_y)) = dragging else {
            return;
        };
        if let Some(l) = layouts.get_mut(id) {
            l.x = (mouse_x - offset_x).max(0.0);
            l.y = (mouse_y - offset_y).max(0.0);
        }
    }

    pub fn apply_resize(
        layouts: &mut HashMap<String, PanelLayout>,
        resizing: &ResizeState,
        mouse_x: f64,
        mouse_y: f64,
    ) {
        let Some((id, start_mx, start_my, start_w, start_h)) = resizing else {
            return;
        };
        if let Some(l) = layouts.get_mut(id) {
            l.w = (start_w + (mouse_x - start_mx)).max(120.0);
            l.h = (start_h + (mouse_y - start_my)).max(80.0);
        }
    }

    // ── End interaction (mouseup / mouseleave) ─────────────────────

    /// Snaps size + finds nearest free position, then clears the interaction state.
    pub fn finish(
        layouts: &mut HashMap<String, PanelLayout>,
        id: &str,
        grid_size: f64,
        margin: f64,
        was_resizing: bool,
    ) {
        if was_resizing {
            if let Some(l) = layouts.get_mut(id) {
                l.w = PanelPlacer::snap(l.w, grid_size);
                l.h = PanelPlacer::snap(l.h, grid_size);
            }
        }

        if let Some(layout) = layouts.get(id).cloned() {
            let (fx, fy) = PanelPlacer::find_nearest_free(id, &layout, layouts, grid_size, margin);
            if let Some(l) = layouts.get_mut(id) {
                l.x = fx;
                l.y = fy;
            }
        }
    }
}
