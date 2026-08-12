use crate::components::dashboard::{
    panel::AsPanel,
    panel_placer::{PanelLayout, PanelPlacer},
};
use web_sys::MouseEvent;

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

    pub fn apply_move(layout: &mut PanelLayout, dragging: &DragState, mouse_x: f64, mouse_y: f64) {
        let Some((_, offset_x, offset_y)) = dragging else {
            return;
        };

        layout.x = (mouse_x - offset_x).max(0.0);
        layout.y = (mouse_y - offset_y).max(0.0);
    }

    pub fn apply_resize(
        layout: &mut PanelLayout,
        resizing: &ResizeState,
        mouse_x: f64,
        mouse_y: f64,
    ) {
        let Some((_, start_mx, start_my, start_w, start_h)) = resizing else {
            return;
        };

        layout.w = (start_w + (mouse_x - start_mx)).max(120.0);
        layout.h = (start_h + (mouse_y - start_my)).max(80.0);
    }

    pub fn finish<P>(panels: &mut [P], id: &str, grid_size: f64, margin: f64, was_resizing: bool)
    where
        P: AsPanel,
    {
        // Snap size if we were resizing
        if was_resizing {
            if let Some(panel) = panels.iter_mut().find(|p| p.id() == id) {
                let layout = panel.layout_mut();
                layout.w = PanelPlacer::snap(layout.w, grid_size);
                layout.h = PanelPlacer::snap(layout.h, grid_size);
            }
        }

        // Get the current panel layout
        let current_layout = panels
            .iter()
            .find(|p| p.id() == id)
            .map(|p| p.layout().clone())
            .unwrap_or(PanelLayout {
                x: 0.0,
                y: 0.0,
                w: 200.0,
                h: 150.0,
            });
        // Get the other panel layouts
        let other_layouts: Vec<PanelLayout> = panels
            .iter()
            .filter(|p| p.id() != id)
            .map(|p| p.layout().clone())
            .collect();

        let other_refs: Vec<&PanelLayout> = other_layouts.iter().collect();

        let (fx, fy) =
            PanelPlacer::find_nearest_free(&current_layout, &other_refs, grid_size, margin);

        // Apply the new position
        if let Some(panel) = panels.iter_mut().find(|p| p.id() == id) {
            panel.layout_mut().x = fx;
            panel.layout_mut().y = fy;
        }
    }
}
