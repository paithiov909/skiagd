use savvy::{savvy, savvy_err, NumericScalar, NumericSexp};
use skia_safe::Paint;

/// PointMode (0-2)
///
/// This is for [add_point()] only.
/// Not used in other functions.
///
/// @seealso
/// [PointMode in skia_safe::canvas - Rust](https://rust-skia.github.io/doc/skia_safe/canvas/enum.PointMode.html)
/// @export
#[savvy]
pub enum PointMode {
    Points,
    Lines,
    Polygon,
}

pub fn sk_point_mode(mode: &PointMode) -> skia_safe::canvas::PointMode {
    match mode {
        PointMode::Points => skia_safe::canvas::PointMode::Points,
        PointMode::Lines => skia_safe::canvas::PointMode::Lines,
        PointMode::Polygon => skia_safe::canvas::PointMode::Polygon,
    }
}

/// Style (0-2)
///
/// @seealso
/// [Style in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Style.html)
/// @export
#[savvy]
pub enum Style {
    StrokeAndFill,
    Stroke,
    Fill,
}

/// Join (0-2)
///
/// @seealso
/// [Join in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Join.html)
/// @export
#[savvy]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

/// Cap (0-2)
///
/// @seealso
/// [Cap in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Cap.html)
/// @export
#[savvy]
pub enum Cap {
    Butt,
    Round,
    Square,
}

/// BlendMode (0-28)
///
/// @seealso
/// [BlendMode in skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/enum.BlendMode.html)
/// @export
#[savvy]
pub enum BlendMode {
    Clear,
    Src,
    Dst,
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcATop,
    DstATop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// PathEffect
///
/// @noRd
#[savvy]
pub struct PathEffect {
    label: String,
    effect: Option<skia_safe::PathEffect>,
}

#[savvy]
impl PathEffect {
    pub fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
    pub fn no_effect() -> savvy::Result<Self> {
        Ok(PathEffect {
            label: "none".to_string(),
            effect: None,
        })
    }
    pub fn trim(start: NumericScalar, end: NumericScalar) -> savvy::Result<Self> {
        let start = start.as_f64();
        let end = end.as_f64();
        if start < 0.0 || start > 1.0 || end < 0.0 || end > 1.0 {
            return Err(savvy_err!("Invalid trim values"));
        }
        let trim_path = skia_safe::PathEffect::trim(
            start as f32,
            end as f32,
            skia_safe::trim_path_effect::Mode::Normal,
        )
        .ok_or_else(|| return savvy_err!("Failed to create path effect"))?;
        Ok(PathEffect {
            label: "trim".to_string(),
            effect: Some(trim_path),
        })
    }
}

/// PaintProps
///
/// Internal impl that wraps `skia_safe::Paint`.
/// Use `PaintProps$set_props()` to create a pointer to PaintProps.
///
/// @details
/// `PaintProps$set_props()` takes arguments below:
///
/// * color: RGBA representaion of a color.
/// * style: Style (stroke style).
/// * join: Join (stroke join).
/// * cap: Cap (stroke cap).
/// * width: Stroke width.
/// * miter: Stroke miter.
/// * blend_mode: BlendMode.
/// * path_effect: PathEffect.
///
/// @noRd
#[savvy]
pub struct PaintProps {
    pub paint: Paint,
}

#[savvy]
impl PaintProps {
    pub fn set_props(
        color: NumericSexp,
        style: Style,
        join: Join,
        cap: Cap,
        width: NumericScalar,
        miter: NumericScalar,
        blend_mode: BlendMode,
        path_effect: PathEffect,
    ) -> savvy::Result<Self> {
        let color = color.as_slice_i32()?;
        if color.len() != 4 {
            return Err(savvy_err!("Invalid color. Expected 4 elements"));
        }
        let width = width.as_f64();
        let miter = miter.as_f64();

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(skia_safe::Color::from_argb(
            color[3] as u8,
            color[0] as u8,
            color[1] as u8,
            color[2] as u8,
        ));
        paint.set_style(sk_style(&style));
        paint.set_stroke_join(sk_join(&join));
        paint.set_stroke_cap(sk_cap(&cap));
        paint.set_stroke_width(width as f32);
        paint.set_stroke_miter(miter as f32);
        paint.set_blend_mode(sk_blend_mode(&blend_mode));
        if let Some(effect) = path_effect.effect {
            paint.set_path_effect(effect);
        }
        Ok(PaintProps { paint })
    }
}

fn sk_style(style: &Style) -> skia_safe::PaintStyle {
    match style {
        Style::StrokeAndFill => skia_safe::PaintStyle::StrokeAndFill,
        Style::Stroke => skia_safe::PaintStyle::Stroke,
        Style::Fill => skia_safe::PaintStyle::Fill,
    }
}

fn sk_join(join: &Join) -> skia_safe::PaintJoin {
    match join {
        Join::Miter => skia_safe::PaintJoin::Miter,
        Join::Round => skia_safe::PaintJoin::Round,
        Join::Bevel => skia_safe::PaintJoin::Bevel,
    }
}

fn sk_cap(cap: &Cap) -> skia_safe::PaintCap {
    match cap {
        Cap::Butt => skia_safe::PaintCap::Butt,
        Cap::Round => skia_safe::PaintCap::Round,
        Cap::Square => skia_safe::PaintCap::Square,
    }
}

fn sk_blend_mode(blend_mode: &BlendMode) -> skia_safe::BlendMode {
    match blend_mode {
        BlendMode::Clear => skia_safe::BlendMode::Clear,
        BlendMode::Src => skia_safe::BlendMode::Src,
        BlendMode::Dst => skia_safe::BlendMode::Dst,
        BlendMode::SrcOver => skia_safe::BlendMode::SrcOver,
        BlendMode::DstOver => skia_safe::BlendMode::DstOver,
        BlendMode::SrcIn => skia_safe::BlendMode::SrcIn,
        BlendMode::DstIn => skia_safe::BlendMode::DstIn,
        BlendMode::SrcOut => skia_safe::BlendMode::SrcOut,
        BlendMode::DstOut => skia_safe::BlendMode::DstOut,
        BlendMode::SrcATop => skia_safe::BlendMode::SrcATop,
        BlendMode::DstATop => skia_safe::BlendMode::DstATop,
        BlendMode::Plus => skia_safe::BlendMode::Plus,
        BlendMode::Xor => skia_safe::BlendMode::Xor,
        BlendMode::Modulate => skia_safe::BlendMode::Modulate,
        BlendMode::Screen => skia_safe::BlendMode::Screen,
        BlendMode::Overlay => skia_safe::BlendMode::Overlay,
        BlendMode::Darken => skia_safe::BlendMode::Darken,
        BlendMode::Lighten => skia_safe::BlendMode::Lighten,
        BlendMode::ColorDodge => skia_safe::BlendMode::ColorDodge,
        BlendMode::ColorBurn => skia_safe::BlendMode::ColorBurn,
        BlendMode::SoftLight => skia_safe::BlendMode::SoftLight,
        BlendMode::HardLight => skia_safe::BlendMode::HardLight,
        BlendMode::Difference => skia_safe::BlendMode::Difference,
        BlendMode::Exclusion => skia_safe::BlendMode::Exclusion,
        BlendMode::Multiply => skia_safe::BlendMode::Multiply,
        BlendMode::Hue => skia_safe::BlendMode::Hue,
        BlendMode::Saturation => skia_safe::BlendMode::Saturation,
        BlendMode::Color => skia_safe::BlendMode::Color,
        BlendMode::Luminosity => skia_safe::BlendMode::Luminosity,
    }
}

// PathEffect
// MaskFilter
// ImageFilter
// ColorFilter
// Shader
