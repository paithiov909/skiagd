use savvy::savvy;

/// @export
#[savvy]
pub struct Shader {
    pub label: String,
    pub shader: Option<skia_safe::Shader>,
}

#[savvy]
impl Shader {
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
}

/// TileMode (0-3)
///
/// `TileMode` determines how the source is tiled for shaders.
/// This is not a paint attribute.
/// To specify `TileMode`, directly pass these pointers to shader functions.
///
/// @rdname skiagd-attrs-tilemode
/// @export
#[savvy]
pub enum TileMode {
    Clamp,
    Repeat,
    Mirror,
    Decal,
}

/// BlendMode (0-28)
///
/// `BlendMode` determines how source and destination colors are combined.
///
/// @details
/// The following blend modes are available in Skia:
///
/// 1. `Clear`
/// 2. `Src`
/// 3. `Dst`
/// 4. `SrcOver`
/// 5. `DstOver`
/// 6. `SrcIn`
/// 7. `DstIn`
/// 8. `SrcOut`
/// 9. `DstOut`
/// 10. `SrcATop`
/// 11. `DstATop`
/// 12. `Xor`
/// 13. `Plus`
/// 14. `Modulate`
/// 15. `Screen`
/// 16. `Overlay`
/// 17. `Darken`
/// 18. `Lighten`
/// 19. `ColorDodge`
/// 20. `ColorBurn`
/// 21. `HardLight`
/// 22. `SoftLight`
/// 23. `Difference`
/// 24. `Exclusion`
/// 25. `Multiply`
/// 26. `Hue`
/// 27. `Saturation`
/// 28. `Color`
/// 29. `Luminosity`
///
/// @seealso
/// [BlendMode in skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/enum.BlendMode.html)
/// @family paint-attributes
/// @rdname skiagd-attrs-blendmode
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

// FIXME: Don't export TileMode. shader functions should take tile mode as string.
pub fn sk_tile_mode(mode: &TileMode) -> skia_safe::TileMode {
    match mode {
        TileMode::Clamp => skia_safe::TileMode::Clamp,
        TileMode::Repeat => skia_safe::TileMode::Repeat,
        TileMode::Mirror => skia_safe::TileMode::Mirror,
        TileMode::Decal => skia_safe::TileMode::Decal,
    }
}

pub fn sk_blend_mode(blend_mode: &BlendMode) -> skia_safe::BlendMode {
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
