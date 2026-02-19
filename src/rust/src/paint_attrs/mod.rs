pub mod font;
mod image_filter;
mod path_effect;
mod shader;
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
/// * blur_style: BlurStyle.
/// * path_effect: PathEffect.
/// * shader: Shader.
/// * image_filter: ImageFilter.
///
/// @noRd
#[savvy]
#[derive(Clone)]
pub struct PaintAttrs {
    pub paint: Paint,
    pub font_size: f32,
    pub font_family: String,
    pub font_face: skia_safe::FontStyle,
    pub blur_style: skia_safe::BlurStyle,
}

#[savvy]
impl PaintAttrs {
    fn set_attrs(
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
        blur_style: &BlurStyle,
        path_effect: &path_effect::PathEffect,
        shader: &shader::Shader,
        image_filter: &image_filter::ImageFilter,
    ) -> savvy::Result<Self> {
        let width = width.as_f64();
        let miter = miter.as_f64();
        let color = num2colors(&color)
            .ok_or_else(|| return savvy_err!("Invalid color. Expected 4 elements"))?;

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(color[0]);
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
        if let Some(filter) = image_filter.filter.clone() {
            paint.set_image_filter(filter);
        }
        Ok(PaintAttrs {
            paint,
            font_size: fontsize.as_f64() as f32,
            font_family: family.to_vec()[0].to_string(),
            font_face: font::sk_font_style(fontface),
            blur_style: sk_blur_style(blur_style),
        })
    }
}

impl PaintAttrs {
    pub fn reset_color(&mut self, color: skia_safe::Color) {
        self.paint.set_color(color);
    }
    pub fn reset_width(&mut self, width: f64) {
        self.paint.set_stroke_width(width as f32);
    }
    pub fn reset_blur(&mut self, sigma: f64) {
        // NOTE: `skia_safe::MaskFilter::blur` returns an option. `None` is returned if the sigma is less than 0.
        self.paint.set_mask_filter(skia_safe::MaskFilter::blur(
            self.blur_style,
            sigma as f32,
            false,
        ));
    }
}

/// Splits a numeric vector into colors
pub fn num2colors(color: &NumericSexp) -> Option<Vec<skia_safe::Color>> {
    let data = color.as_slice_f64();
    let mut ret = Vec::new();
    for chunk in data.chunks(4) {
        if chunk.len() == 4 {
            ret.push(skia_safe::Color::from_argb(
                chunk[3] as u8,
                chunk[0] as u8,
                chunk[1] as u8,
                chunk[2] as u8,
            ));
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

/// If actual length is not equal to expected, raises an error
pub fn assert_len(name: &str, expected: usize, actual: usize) -> anyhow::Result<(), savvy::Error> {
    if actual != expected {
        Err(savvy_err!("{} must have {} elements", name, expected))
    } else {
        Ok(())
    }
}

/// Get width, bbox and number of characters
///
/// @param text Text strings.
/// @param props PaintAttrs.
/// @returns A list.
/// @noRd
#[savvy]
fn sk_get_text_info(text: savvy::StringSexp, props: PaintAttrs) -> savvy::Result<savvy::Sexp> {
    let typeface = font::match_family_style(props.font_family.as_str(), props.font_face)?;
    let font = skia_safe::Font::from_typeface(&typeface, props.font_size);

    let mut id = savvy::OwnedIntegerSexp::new(text.len())?;
    let mut n_chars = savvy::OwnedIntegerSexp::new(text.len())?;
    let mut width = savvy::OwnedRealSexp::new(text.len())?;
    let mut l = savvy::OwnedIntegerSexp::new(text.len())?;
    let mut t = savvy::OwnedIntegerSexp::new(text.len())?;
    let mut r = savvy::OwnedIntegerSexp::new(text.len())?;
    let mut b = savvy::OwnedIntegerSexp::new(text.len())?;
    for (i, txt) in text.iter().enumerate() {
        let glyph_ids = font.text_to_glyphs_vec(txt.to_string());
        let n = font.count_text(txt.to_string());
        let (w, rect) = font.measure_text(glyph_ids.as_slice(), Some(&props.paint));
        id.set_elt(i, i as i32)?;
        n_chars.set_elt(i, n as i32)?;
        width.set_elt(i, w as f64)?;
        l.set_elt(i, rect.left() as i32)?;
        t.set_elt(i, rect.top() as i32)?;
        r.set_elt(i, rect.right() as i32)?;
        b.set_elt(i, rect.bottom() as i32)?;
    }
    let mut out = savvy::OwnedListSexp::new(7, true)?;
    out.set_name_and_value(0, "id", id)?;
    out.set_name_and_value(1, "n_chars", n_chars)?;
    out.set_name_and_value(2, "advance_width", width)?;
    out.set_name_and_value(3, "l", l)?;
    out.set_name_and_value(4, "t", t)?;
    out.set_name_and_value(5, "r", r)?;
    out.set_name_and_value(6, "b", b)?;
    Ok(out.into())
}

/// BlurStyle (0-3)
///
/// `BlurStyle` controls how a blur mask filter is applied to the shape.
///
/// @details
/// The following `BlurStyle` are available:
///
/// * `Normal`: Normal blur.
/// * `Solid`: Solid blur.
/// * `Outer`: Outer blur.
/// * `Inner`: Inner blur.
///
/// @seealso
/// [BlurStyle in skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/enum.BlurStyle.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-blurstyle
/// @export
#[savvy]
pub enum BlurStyle {
    Normal,
    Solid,
    Outer,
    Inner,
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

fn sk_blur_style(style: &BlurStyle) -> skia_safe::BlurStyle {
    match style {
        BlurStyle::Normal => skia_safe::BlurStyle::Normal,
        BlurStyle::Solid => skia_safe::BlurStyle::Solid,
        BlurStyle::Outer => skia_safe::BlurStyle::Outer,
        BlurStyle::Inner => skia_safe::BlurStyle::Inner,
    }
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

pub fn sk_fill_type(mode: &FillType) -> skia_safe::PathFillType {
    match mode {
        FillType::Winding => skia_safe::PathFillType::Winding,
        FillType::EvenOdd => skia_safe::PathFillType::EvenOdd,
        FillType::InverseWinding => skia_safe::PathFillType::InverseWinding,
        FillType::InverseEvenOdd => skia_safe::PathFillType::InverseEvenOdd,
    }
}
