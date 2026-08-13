use crate::components::dashboard::{
    panel::{Panel, PanelInteraction},
    panel_placer::{PanelLayout, PanelPlacer},
};

use web_sys::MouseEvent;

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

    // ── Continuous update (mousemove) ──────────────────────────────
    pub fn apply(
        panels: &mut Vec<Panel>,
        interaction: &PanelInteraction,
        mouse_x: f64,
        mouse_y: f64,
    ) {
        match interaction {
            PanelInteraction::Dragging {
                id,
                offset_x,
                offset_y,
            } => {
                if let Some(panel) = panels.iter_mut().find(|p| p.id == *id) {
                    panel.layout.x = (mouse_x - offset_x).max(0.0);
                    panel.layout.y = (mouse_y - offset_y).max(0.0);
                }
            }

            PanelInteraction::Resizing {
                id,
                start_mx,
                start_my,
                start_w,
                start_h,
            } => {
                if let Some(panel) = panels.iter_mut().find(|p| p.id == *id) {
                    panel.layout.w = (start_w + (mouse_x - start_mx)).max(120.0);

                    panel.layout.h = (start_h + (mouse_y - start_my)).max(80.0);
                }
            }

            PanelInteraction::Removing { .. } => return,
        }
    }

    pub fn finish(
        panels: &mut Vec<Panel>,
        interaction: &PanelInteraction,
        grid_size: f64,
        margin: f64,
    ) {
        let (id, was_resizing) = match interaction {
            PanelInteraction::Dragging { id, .. } => (id, false),
            PanelInteraction::Resizing { id, .. } => (id, true),
            PanelInteraction::Removing { .. } => return,
        };

        let Some(index) = panels.iter().position(|p| p.id == *id) else {
            return;
        };

        let mut current = panels[index].layout.clone();

        if was_resizing {
            current.w = PanelPlacer::snap(current.w, grid_size);
            current.h = PanelPlacer::snap(current.h, grid_size);
        }

        // This section belongs elsewhere
        let others: Vec<&PanelLayout> = panels
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, p)| &p.layout)
            .collect();

        let (fx, fy) = PanelPlacer::find_nearest_free(&current, &others, grid_size, margin);
        // --------------------------------
        current.x = fx;
        current.y = fy;

        panels[index].layout = current;
    }
}
