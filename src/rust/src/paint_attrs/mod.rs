pub mod path_effect;
pub mod shader;
mod stroke;

use savvy::{savvy, savvy_err, NumericScalar, NumericSexp};
use skia_safe::Paint;

/// PaintAttrs
///
/// Internal impl that wraps `skia_safe::Paint`.
/// Use `PaintAttrs$set_attrs()` to create a pointer to PaintAttrs.
///
/// @details
/// `PaintAttrs$set_attrs()` takes arguments below:
///
/// * color: RGBA representaion of a color.
/// * style: Style (stroke style).
/// * join: Join (stroke join).
/// * cap: Cap (stroke cap).
/// * width: Stroke width.
/// * miter: Stroke miter.
/// * blend_mode: BlendMode.
/// * path_effect: PathEffect.
/// * shader: Shader.
///
/// @noRd
#[savvy]
pub struct PaintAttrs {
    pub paint: Paint,
}

#[savvy]
impl PaintAttrs {
    pub fn set_attrs(
        color: NumericSexp,
        style: &stroke::Style,
        join: &stroke::Join,
        cap: &stroke::Cap,
        width: NumericScalar,
        miter: NumericScalar,
        blend_mode: &shader::BlendMode,
        path_effect: &path_effect::PathEffect,
        shader: &shader::Shader,
    ) -> savvy::Result<Self> {
        if color.len() != 4 {
            return Err(savvy_err!("Invalid color. Expected 4 elements"));
        }
        let width = width.as_f64();
        let miter = miter.as_f64();
        let color = color.as_slice_f64();

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(skia_safe::Color::from_argb(
            color[3] as u8,
            color[0] as u8,
            color[1] as u8,
            color[2] as u8,
        ));
        paint.set_style(stroke::sk_style(&style));
        paint.set_stroke_join(stroke::sk_join(&join));
        paint.set_stroke_cap(stroke::sk_cap(&cap));
        paint.set_stroke_width(width as f32);
        paint.set_stroke_miter(miter as f32);
        paint.set_blend_mode(shader::sk_blend_mode(&blend_mode));
        if let Some(effect) = path_effect.effect.clone() {
            paint.set_path_effect(effect);
        }
        if let Some(shader) = shader.shader.clone() {
            paint.set_shader(shader);
        }
        Ok(PaintAttrs { paint })
    }
}

/// PointMode (0-2)
///
/// `PointMode` determines how points are drawn.
/// This is for [add_point()] only. Not used in other functions.
///
/// @details
/// The following `PointMode` are available:
///
/// * `Points`: Draws each `point` as a point. The shape of point drawn depends on `props`.
/// * `Lines`: Each pair of `point` draws a line segment. One line is drawn for every two points; each point is used once. If count is odd, the final point is ignored.
/// * `Polygon`: Each adjacent pair of `point` draws a line segment. count minus one lines are drawn; the first and last point are used once.
///
/// @seealso
/// [PointMode in skia_safe::canvas - Rust](https://rust-skia.github.io/doc/skia_safe/canvas/enum.PointMode.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-pointmode
/// @export
#[savvy]
pub enum PointMode {
    Points,
    Lines,
    Polygon,
}

/// FillType (0-3)
///
/// `FillType` determines how paths are drawn.
/// This is for [add_path()] only. Not used in other functions.
///
/// @details
/// The following `FillType` are available:
///
/// * `Winding`
/// * `EvenOdd`
/// * `InverseWinding`
/// * `InverseEvenOdd`
///
/// @seealso
/// [FillType in skia_safe::path - Rust](https://rust-skia.github.io/doc/skia_safe/path/enum.FillType.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-filltype
/// @export
#[savvy]
pub enum FillType {
    Winding,
    EvenOdd,
    InverseWinding,
    InverseEvenOdd,
}

pub fn sk_point_mode(mode: &PointMode) -> skia_safe::canvas::PointMode {
    match mode {
        PointMode::Points => skia_safe::canvas::PointMode::Points,
        PointMode::Lines => skia_safe::canvas::PointMode::Lines,
        PointMode::Polygon => skia_safe::canvas::PointMode::Polygon,
    }
}

pub fn sk_fill_type(mode: &FillType) -> skia_safe::path::FillType {
    match mode {
        FillType::Winding => skia_safe::path::FillType::Winding,
        FillType::EvenOdd => skia_safe::path::FillType::EvenOdd,
        FillType::InverseWinding => skia_safe::path::FillType::InverseWinding,
        FillType::InverseEvenOdd => skia_safe::path::FillType::InverseEvenOdd,
    }
}
