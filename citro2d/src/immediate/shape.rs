//! Wrappers for stateless C2D_Draw* functions (2D shapes).

use crate::{Color, GradientColor, Point, Size};

/* Circle */

/// Draws a circle with a gradient color.
pub fn draw_circle_gradient(
    center: impl Into<Point>,
    radius: f32,
    gradient_clr: GradientColor,
) -> bool {
    let center = center.into();
    unsafe {
        citro2d_sys::C2D_DrawCircle(
            center.x,
            center.y,
            0.,
            radius,
            gradient_clr.top_l.inner,
            gradient_clr.top_r.inner,
            gradient_clr.bot_l.inner,
            gradient_clr.bot_r.inner,
        )
    }
}

/// Draws a circle with a gradient color and a border.
pub fn draw_circle_gradient_border(
    center: impl Into<Point> + Copy,
    radius: f32,
    gradient_clr: GradientColor,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    draw_circle_solid(center, radius + border_thickness, border_clr)
        && draw_circle_gradient(center, radius, gradient_clr)
}

/// Draws a circle with a solid color.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
#[inline(always)]
pub fn draw_circle_solid(center: impl Into<Point>, radius: f32, clr: impl Into<Color>) -> bool {
    let center = center.into();
    let clr = clr.into();
    unsafe { citro2d_sys::C2D_DrawCircleSolid(center.x, center.y, center.z, radius, clr.inner) }
}

/// Draws a circle with a solid color and a border.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
pub fn draw_circle_solid_border(
    center: impl Into<Point> + Copy,
    radius: f32,
    fill_clr: impl Into<Color>,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    draw_circle_solid(center, radius + border_thickness, border_clr)
        && draw_circle_solid(center, radius, fill_clr)
}

/* Ellipse */

/// Draws an ellipse with a gradient color.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
#[inline(always)]
pub fn draw_ellipse_gradient(
    top_left: impl Into<Point>,
    size: impl Into<Size>,
    gradient_clr: GradientColor,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    unsafe {
        citro2d_sys::C2D_DrawEllipse(
            top_left.x,
            top_left.y,
            top_left.z,
            size.width,
            size.height,
            gradient_clr.top_l.inner,
            gradient_clr.top_r.inner,
            gradient_clr.bot_l.inner,
            gradient_clr.bot_r.inner,
        )
    }
}

/// Draws an ellipse with a gradient color and a border.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
pub fn draw_ellipse_gradient_border(
    top_left: impl Into<Point> + Copy,
    size: impl Into<Size> + Copy,
    gradient_clr: GradientColor,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    draw_ellipse_solid(
        tuple_add(top_left.into(), -border_thickness),
        tuple_add(size.into(), border_thickness + border_thickness),
        border_clr,
    ) && draw_ellipse_gradient(top_left, size, gradient_clr)
}

/// Draws an ellipse with a solid color.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
#[inline(always)]
pub fn draw_ellipse_solid(
    center: impl Into<Point>,
    size: impl Into<Size>,
    clr: impl Into<Color>,
) -> bool {
    let center = center.into();
    let size = size.into();
    let clr = clr.into();

    unsafe {
        citro2d_sys::C2D_DrawEllipseSolid(
            center.x,
            center.y,
            center.z,
            size.width,
            size.height,
            clr.inner,
        )
    }
}

/// Draws an ellipse with a solid color and a border.
///
/// FIXME: This has weird side effects: no rectangles will be drawn afterward and sprites will appear black.
pub fn draw_ellipse_solid_border(
    center: impl Into<Point> + Copy,
    size: impl Into<Size> + Copy,
    fill_clr: impl Into<Color>,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    let size = size.into();
    draw_ellipse_solid(
        center,
        tuple_add(size.into(), border_thickness + border_thickness),
        border_clr,
    ) && draw_ellipse_solid(center, size, fill_clr)
}

/* Triangle */

/// Draws a triangle defined by three vertices with a gradient color spanning from each vertex.
#[inline(always)]
pub fn draw_triangle_gradient(
    vtx0: impl Into<Point>,
    vtx1: impl Into<Point>,
    vtx2: impl Into<Point>,
    clr0: impl Into<Color>,
    clr1: impl Into<Color>,
    clr2: impl Into<Color>,
) -> bool {
    let (vtx0, vtx1, vtx2) = (vtx0.into(), vtx1.into(), vtx2.into());
    let (clr0, clr1, clr2) = (clr0.into(), clr1.into(), clr2.into());
    unsafe {
        citro2d_sys::C2D_DrawTriangle(
            vtx0.x, vtx0.y, clr0.inner, vtx1.x, vtx1.y, clr1.inner, vtx2.x, vtx2.y, clr2.inner,
            vtx0.z,
        )
    }
}

/// Draws a triangle with a gradient color and border.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn draw_triangle_gradient_border(
    vtx0: impl Into<Point> + Copy,
    vtx1: impl Into<Point> + Copy,
    vtx2: impl Into<Point> + Copy,
    clr0: impl Into<Color>,
    clr1: impl Into<Color>,
    clr2: impl Into<Color>,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    let (b_vtx0, b_vtx1, b_vtx2) =
        scale_triangle(vtx0.into(), vtx1.into(), vtx2.into(), border_thickness);
    draw_triangle_solid(b_vtx0, b_vtx1, b_vtx2, border_clr)
        && draw_triangle_gradient(vtx0, vtx1, vtx2, clr0, clr1, clr2)
}

// N.B. there is no libcitro2d function for a solid triangle.
/// Draws a triangle defined by three vertices with a solid color.
#[inline(always)]
pub fn draw_triangle_solid(
    vtx0: impl Into<Point>,
    vtx1: impl Into<Point>,
    vtx2: impl Into<Point>,
    clr: impl Into<Color>,
) -> bool {
    let (vtx0, vtx1, vtx2) = (vtx0.into(), vtx1.into(), vtx2.into());
    let clr = clr.into();
    unsafe {
        citro2d_sys::C2D_DrawTriangle(
            vtx0.x, vtx0.y, clr.inner, vtx1.x, vtx1.y, clr.inner, vtx2.x, vtx2.y, clr.inner, vtx0.z,
        )
    }
}

/// Draws a triangle with a solid color and border.
#[inline(always)]
pub fn draw_triangle_solid_border(
    vtx0: impl Into<Point> + Copy,
    vtx1: impl Into<Point> + Copy,
    vtx2: impl Into<Point> + Copy,
    fill_clr: impl Into<Color>,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    let (b_vtx0, b_vtx1, b_vtx2) =
        scale_triangle(vtx0.into(), vtx1.into(), vtx2.into(), border_thickness);
    draw_triangle_solid(b_vtx0, b_vtx1, b_vtx2, border_clr)
        && draw_triangle_solid(vtx0, vtx1, vtx2, fill_clr)
}

/* Rectangle */

/// Draws a rectangle with a gradient color.
pub fn draw_rect_gradient(
    top_left: impl Into<Point>,
    size: impl Into<Size>,
    gradient_clr: GradientColor,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    unsafe {
        citro2d_sys::C2D_DrawRectangle(
            top_left.x,
            top_left.y,
            top_left.z,
            size.width,
            size.height,
            gradient_clr.top_l.inner,
            gradient_clr.top_r.inner,
            gradient_clr.bot_l.inner,
            gradient_clr.bot_r.inner,
        )
    }
}

/// Draws a rectangle with a gradient color and a border.
pub fn draw_rect_gradient_border(
    top_left: impl Into<Point> + Copy,
    size: impl Into<Size>,
    fill_clr: GradientColor,
    border_clr: Color,
    border_thickness: f32,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    draw_rect_solid(
        tuple_add(top_left.into(), -border_thickness),
        tuple_add(size.into(), border_thickness + border_thickness),
        border_clr,
    ) && draw_rect_gradient(top_left, size, fill_clr)
}

/// Draws a rectangle with a solid color.
#[inline(always)]
pub fn draw_rect_solid(
    top_left: impl Into<Point>,
    size: impl Into<Size>,
    clr: impl Into<Color>,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    let clr = clr.into();
    unsafe {
        citro2d_sys::C2D_DrawRectSolid(
            top_left.x,
            top_left.y,
            top_left.z,
            size.width,
            size.height,
            clr.inner,
        )
    }
}

/// Draws a rectangle with a solid color and a border.
pub fn draw_rect_solid_border(
    top_left: impl Into<Point>,
    size: impl Into<Size>,
    fill_clr: impl Into<Color>,
    border_clr: impl Into<Color>,
    border_thickness: f32,
) -> bool {
    let top_left = top_left.into();
    let size = size.into();
    draw_rect_solid(
        tuple_add(top_left.into(), -border_thickness),
        tuple_add(size.into(), border_thickness + border_thickness),
        border_clr,
    ) && draw_rect_solid(top_left, size, fill_clr)
}

/* Line */

/// Draws a line with a gradient spanning between two vertices.
#[inline(always)]
pub fn draw_line_gradient(
    vtx0: Point,
    vtx1: Point,
    clr0: Color,
    clr1: Color,
    thickness: f32,
) -> bool {
    unsafe {
        citro2d_sys::C2D_DrawLine(
            vtx0.x, vtx0.y, clr0.inner, vtx1.x, vtx1.y, clr1.inner, thickness, vtx0.z,
        )
    }
}

/// Draws a solid line spanning between two vertices.
#[inline(always)]
pub fn draw_line_solid(vtx0: Point, vtx1: Point, clr: Color, thickness: f32) -> bool {
    unsafe {
        citro2d_sys::C2D_DrawLine(
            vtx0.x, vtx0.y, clr.inner, vtx1.x, vtx1.y, clr.inner, thickness, vtx0.z,
        )
    }
}

/// Helper function to add a value to each field of a (f32, f32) tuple.
fn tuple_add(mut tuple: (f32, f32), value: f32) -> (f32, f32) {
    tuple.0 += value;
    tuple.1 += value;
    tuple
}

/// Helper function to scale a triangle for rendering its border.
fn scale_triangle(vtx0: Point, vtx1: Point, vtx2: Point, px: f32) -> (Point, Point, Point) {
    let centroid_x = (vtx0.x + vtx1.x + vtx2.x) / 3.;
    let centroid_y = (vtx0.y + vtx1.y + vtx2.y) / 3.;

    // fixme: Scuffed way to scale by a pixel quantity:
    // We get the size of the triangle's bounding box and find the ratio between
    // the "border triangle"'s bounding box and it.

    let min_x = vtx0.x.min(vtx1.x).min(vtx2.x);
    let max_x = vtx0.x.max(vtx1.x).max(vtx2.x);
    let w = max_x - min_x;
    let scale_x = (px + w) / w;

    let min_y = vtx0.y.min(vtx1.y).min(vtx2.y);
    let max_y = vtx0.y.max(vtx1.y).max(vtx2.y);
    let h = max_y - min_y;
    let scale_y = (px + h) / h;

    let scale = (scale_x + scale_y) / 2.;

    let [b_vtx0, b_vtx1, b_vtx2] = [vtx0, vtx1, vtx2].map(|Point { x, y, z }| {
        (
            centroid_x + (x - centroid_x) * scale,
            centroid_y + (y - centroid_y) * scale,
            z,
        )
            .into()
    });

    (b_vtx0, b_vtx1, b_vtx2)
}
