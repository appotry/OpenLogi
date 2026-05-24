//! Canvas-painted leader lines from each hotspot to its side-label anchor.
//!
//! Per UI.md Phase 7. Each polyline is hotspot-centre → short horizontal
//! stub → diagonal to the label anchor. The active hotspot's line is
//! coloured blue and stroked thicker; everything else stays muted.

use gpui::{Bounds, PathBuilder, Pixels, Point, Window, hsla, point, px, rgb};

use crate::data::mouse_buttons::{ButtonId, Hotspot};
use crate::theme::ACCENT_BLUE;

/// Length of the horizontal stub before turning toward the label.
const STUB: f32 = 32.;

/// Which side of the mouse a label sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Label {
    pub id: ButtonId,
    pub side: Side,
    /// Y of the label anchor, in mouse-canvas coords.
    pub y: f32,
}

/// Paint every leader line. `mouse_origin` is the top-left of the mouse
/// silhouette inside the canvas; hotspot coords are mouse-local, label
/// coords are canvas-local.
pub fn paint(
    canvas_bounds: Bounds<Pixels>,
    mouse_origin: Point<Pixels>,
    mouse_w: f32,
    hotspots: &[Hotspot],
    labels: &[Label],
    highlighted: Option<ButtonId>,
    window: &mut Window,
) {
    for label in labels {
        let Some(hotspot) = hotspots.iter().find(|h| h.id == label.id) else {
            continue;
        };
        paint_one(
            canvas_bounds,
            mouse_origin,
            mouse_w,
            *hotspot,
            *label,
            highlighted == Some(label.id),
            window,
        );
    }
}

fn paint_one(
    canvas_bounds: Bounds<Pixels>,
    mouse_origin: Point<Pixels>,
    mouse_w: f32,
    hotspot: Hotspot,
    label: Label,
    highlight: bool,
    window: &mut Window,
) {
    let (hx, hy) = hotspot.center();
    // Hotspot centre in canvas-local coords (the path is painted with
    // origin = canvas_bounds.origin removed).
    let hotspot_centre = mouse_origin + point(px(hx), px(hy));

    // Where the polyline turns horizontally outward before angling toward
    // the label anchor.
    let (stub_x, anchor_x) = match label.side {
        Side::Left => (
            mouse_origin.x + px(0.) - px(STUB),
            mouse_origin.x - px(STUB) - px(160.),
        ),
        Side::Right => (
            mouse_origin.x + px(mouse_w) + px(STUB),
            mouse_origin.x + px(mouse_w) + px(STUB) + px(160.),
        ),
    };
    let stub_y = hotspot_centre.y;
    let anchor_y = mouse_origin.y + px(label.y);

    let stub = Point {
        x: stub_x,
        y: stub_y,
    };
    let anchor = Point {
        x: anchor_x,
        y: anchor_y,
    };

    let width = if highlight { px(2.5) } else { px(1.) };
    let mut path = PathBuilder::stroke(width);
    path.move_to(hotspot_centre - canvas_bounds.origin);
    path.line_to(stub - canvas_bounds.origin);
    path.line_to(anchor - canvas_bounds.origin);

    if let Ok(built) = path.build() {
        if highlight {
            window.paint_path(built, rgb(ACCENT_BLUE));
        } else {
            // Muted gray — readable against the dark background without
            // competing with the highlighted line.
            window.paint_path(built, hsla(0., 0., 0.5, 0.45));
        }
    }
}
