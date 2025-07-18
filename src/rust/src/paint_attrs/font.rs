use savvy::{savvy, savvy_err};

#[savvy]
fn sk_list_families() -> savvy::Result<savvy::Sexp> {
    let mgr = skia_safe::FontMgr::new();
    let families = mgr.family_names();
    let mut ret: Vec<String> = Vec::new();
    for f in families {
        ret.push(f);
    }
    ret.try_into()
}

/// Takes a font family and style and returns a `skia_safe::Typeface`
pub fn match_family_style(
    family: &str,
    style: skia_safe::FontStyle,
) -> anyhow::Result<skia_safe::Typeface, savvy::Error> {
    let mgr = skia_safe::FontMgr::new();
    let typeface = mgr
        .match_family_style(family, style)
        .ok_or_else(|| return savvy_err!("Font family '{}' not found", family))?;
    Ok(typeface)
}

/// FontStyle (0-3)
///
/// `FontStyle` determines the font style.
///
/// @details
/// The following styles are available:
///
/// * `Normal`: Normal (plain).
/// * `Bold`: Bold (bold).
/// * `Italic`: Italic (italic).
/// * `BoldItalic`: BoldItalic (bold.italic).
///
/// @seealso
/// [FontStyle in skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/struct.FontStyle.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-fontstyle
/// @export
#[savvy]
pub enum FontStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}

pub fn sk_font_style(style: &FontStyle) -> skia_safe::FontStyle {
    match style {
        FontStyle::Normal => skia_safe::FontStyle::normal(),
        FontStyle::Bold => skia_safe::FontStyle::bold(),
        FontStyle::Italic => skia_safe::FontStyle::italic(),
        FontStyle::BoldItalic => skia_safe::FontStyle::bold_italic(),
    }
}
