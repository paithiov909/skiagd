pub mod font;
pub mod path_effect;
pub mod shader;
mod stroke;

use savvy::{savvy, savvy_err, NumericScalar, NumericSexp, StringSexp};
use skia_safe::Paint;

/// PaintAttrs
///
/// Internal impl that wraps `skia_safe::Paint`.
/// Use `PaintAttrs$set_attrs()` to create a new PaintAttrs
/// each time it is required, since `props` is always moved.
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
/// * fontsize: Font size.
/// * family: Font family name
/// * fontface: FontStyle.
/// * blend_mode: BlendMode.
/// * path_effect: PathEffect.
/// * shader: Shader.
///
/// @noRd
#[savvy]
pub struct PaintAttrs {
    pub paint: Paint,
    pub font_size: f32,
    pub font_family: String,
    pub font_face: skia_safe::FontStyle,
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
        fontsize: NumericScalar,
        family: StringSexp,
        fontface: &font::FontStyle,
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
        Ok(PaintAttrs {
            paint,
            font_size: fontsize.as_f64() as f32,
            font_family: family.to_vec()[0].to_string(),
            font_face: font::sk_font_style(fontface),
        })
    }
}

/// Get text width
///
/// @param text Text strings.
/// @param props PaintAttrs.
/// @returns A numeric vector.
/// @noRd
#[savvy]
fn sk_get_text_width(
    text: savvy::StringSexp,
    props: PaintAttrs,
) -> savvy::Result<savvy::Sexp> {
    let typeface = font::match_family_style(props.font_family.as_str(), props.font_face)?;
    let font = skia_safe::Font::from_typeface(&typeface, props.font_size);
    let mut out = savvy::OwnedRealSexp::new(text.len())?;
    for (i, t) in text.iter().enumerate() {
        let ids = font.text_to_glyphs_vec(t.to_string());
        let mut num_ids: Vec<f32> = Vec::new();
        num_ids.resize(font.count_text(t.to_string()), 0.0);
        let width_ptrs = num_ids.as_mut_slice();
        font.get_widths_bounds(ids.as_slice(), Some(width_ptrs), None, Some(&props.paint));
        out.set_elt(i, width_ptrs.iter().map(|x| *x as f64).sum::<f64>())?;
    }
    out.into()
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


/// VertexMode (0-2)
///
/// `VertexMode` determines how vertices are drawn.
/// This is for [add_vertices()] only. Not used in other functions.
///
/// @details
/// The following `VertexMode` are available:
///
/// * `Triangles`
/// * `TriangleStrip`
/// * `TriangleFan`
///
/// @seealso
/// [VertexMode in skia_safe::vertices - Rust](https://rust-skia.github.io/doc/skia_safe/vertices/enum.VertexMode.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-vertexmode
/// @export
#[savvy]
pub enum VertexMode {
    Triangles,
    TriangleStrip,
    TriangleFan,
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

pub fn sk_vertex_mode(mode: &VertexMode) -> skia_safe::vertices::VertexMode {
    match mode {
        VertexMode::Triangles => skia_safe::vertices::VertexMode::Triangles,
        VertexMode::TriangleStrip => skia_safe::vertices::VertexMode::TriangleStrip,
        VertexMode::TriangleFan => skia_safe::vertices::VertexMode::TriangleFan,
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
