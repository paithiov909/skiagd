use savvy::savvy;

/// Style (0-2)
///
/// `Style` determines the stroke style of shapes.
///
/// @details
/// The following styles are available:
///
/// * `StrokeAndFill`: Stroke and fill.
/// * `Stroke`: Stroke only.
/// * `Fill`: Fill only.
///
/// @seealso
/// [Style in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Style.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-style
/// @export
#[savvy]
pub enum Style {
    StrokeAndFill,
    Stroke,
    Fill,
}

/// Join (0-2)
///
/// `Join` determines the stroke join (the geometry drawn at the corners of strokes) for shapes.
///
/// @details
/// The following joins are available:
///
/// * `Miter`: Miter join.
/// * `Round`: Round join.
/// * `Bevel`: Bevel join.
///
/// @seealso
/// [Join in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Join.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-join
/// @export
#[savvy]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

/// Cap (0-2)
///
/// `Cap` determines the stroke cap (the geometry drawn at the beginning and end of strokes).
///
/// @details
/// The following caps are available:
///
/// * `Butt`: Butt cap.
/// * `Round`: Round cap.
/// * `Square`: Square cap.
///
/// @seealso
/// [Cap in skia_safe::paint - Rust](https://rust-skia.github.io/doc/skia_safe/paint/enum.Cap.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-cap
/// @export
#[savvy]
pub enum Cap {
    Butt,
    Round,
    Square,
}


pub fn sk_style(style: &Style) -> skia_safe::PaintStyle {
  match style {
      Style::StrokeAndFill => skia_safe::PaintStyle::StrokeAndFill,
      Style::Stroke => skia_safe::PaintStyle::Stroke,
      Style::Fill => skia_safe::PaintStyle::Fill,
  }
}

pub fn sk_join(join: &Join) -> skia_safe::PaintJoin {
  match join {
      Join::Miter => skia_safe::PaintJoin::Miter,
      Join::Round => skia_safe::PaintJoin::Round,
      Join::Bevel => skia_safe::PaintJoin::Bevel,
  }
}

pub fn sk_cap(cap: &Cap) -> skia_safe::PaintCap {
  match cap {
      Cap::Butt => skia_safe::PaintCap::Butt,
      Cap::Round => skia_safe::PaintCap::Round,
      Cap::Square => skia_safe::PaintCap::Square,
  }
}
