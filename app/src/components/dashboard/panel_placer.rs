use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct PanelLayout {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub struct PanelPlacer;

impl PanelPlacer {
    pub fn snap(value: f64, grid_size: f64) -> f64 {
        (value / grid_size).round() * grid_size
    }

    pub fn overlaps(a: &PanelLayout, b: &PanelLayout, margin: f64) -> bool {
        let ax2 = a.x + a.w + margin;
        let ay2 = a.y + a.h + margin;
        let bx2 = b.x + b.w + margin;
        let by2 = b.y + b.h + margin;

        a.x < bx2 && ax2 > b.x && a.y < by2 && ay2 > b.y
    }

    pub fn find_nearest_free(
        id: &str,
        layout: &PanelLayout,
        layouts: &HashMap<String, PanelLayout>,
        grid_size: f64,
        margin: f64,
    ) -> (f64, f64) {
        let others: Vec<&PanelLayout> = layouts
            .iter()
            .filter(|(other_id, _)| *other_id != id)
            .map(|(_, layout)| layout)
            .collect();

        if others
            .iter()
            .all(|other| !Self::overlaps(layout, other, margin))
        {
            return (layout.x, layout.y);
        }

        for step in 1..=40 {
            let dist = step as f64 * grid_size;

            let candidates = [
                (layout.x + dist, layout.y),
                (layout.x - dist, layout.y),
                (layout.x, layout.y + dist),
                (layout.x, layout.y - dist),
                (layout.x + dist, layout.y + dist),
                (layout.x + dist, layout.y - dist),
                (layout.x - dist, layout.y + dist),
                (layout.x - dist, layout.y - dist),
            ];

            for (cx, cy) in candidates {
                let test = PanelLayout {
                    x: Self::snap(cx.max(0.0), grid_size),
                    y: Self::snap(cy.max(0.0), grid_size),
                    w: layout.w,
                    h: layout.h,
                };

                if others
                    .iter()
                    .all(|other| !Self::overlaps(&test, other, margin))
                {
                    return (test.x, test.y);
                }
            }
        }

        (
            Self::snap(layout.x.max(0.0), grid_size),
            Self::snap(layout.y.max(0.0), grid_size),
        )
    }
}
