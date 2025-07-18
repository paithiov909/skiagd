use super::{assert_len, num2colors};
use crate::canvas::read_picture_bytes;
use crate::path_transform::as_matrix;
use crate::runtime_effect;

use savvy::{savvy, savvy_err, LogicalSexp, NumericScalar, NumericSexp};
use skia_safe::{Data, Image};

/// @export
#[savvy]
pub struct Shader {
    label: String,
    pub shader: Option<skia_safe::Shader>,
}

#[savvy]
impl Shader {
    fn no_shader() -> savvy::Result<Self> {
        Ok(Shader {
            label: "none".to_string(),
            shader: None,
        })
    }
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
    fn color(rgba: NumericSexp) -> savvy::Result<Self> {
        let color = num2colors(&rgba)
            .ok_or_else(|| return savvy_err!("Invalid color. Expected 4 elements"))?;
        Ok(Shader {
            label: "color".to_string(),
            shader: Some(skia_safe::shader::shaders::color(color[0])),
        })
    }
    fn blend(mode: BlendMode, dst: &Shader, src: &Shader) -> savvy::Result<Self> {
        let dst = dst
            .shader
            .clone()
            .ok_or(savvy_err!("dst shader is required"))?;
        let src = src
            .shader
            .clone()
            .ok_or(savvy_err!("src shader is required"))?;
        let shader_blend = skia_safe::shader::shaders::blend(
            skia_safe::Blender::from(sk_blend_mode(&mode)),
            dst,
            src,
        );
        Ok(Shader {
            label: "blend".to_string(),
            shader: Some(shader_blend),
        })
    }
    fn fractal_noise(
        freq: NumericSexp,
        octaves: NumericScalar,
        seed: NumericScalar,
        tile_size: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("freq", 2, freq.len())?;
        assert_len("tile_size", 2, tile_size.len())?;

        let freq = freq.as_slice_f64();
        let octaves = octaves.as_usize()?;
        let seed = seed.as_f64();
        let tile_size = tile_size.as_slice_f64();
        let shader_fractal_noise = skia_safe::Shader::fractal_perlin_noise(
            (freq[0] as f32, freq[1] as f32),
            octaves,
            seed as f32,
            Some(skia_safe::ISize::new(
                tile_size[0] as i32,
                tile_size[1] as i32,
            )),
        );
        Ok(Shader {
            label: "fractal_noise".to_string(),
            shader: shader_fractal_noise,
        })
    }
    fn turbulence(
        freq: NumericSexp,
        octaves: NumericScalar,
        seed: NumericScalar,
        tile_size: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("freq", 2, freq.len())?;
        assert_len("tile_size", 2, tile_size.len())?;

        let freq = freq.as_slice_f64();
        let octaves = octaves.as_usize()?;
        let seed = seed.as_f64();
        let tile_size = tile_size.as_slice_f64();
        let shader_turbulence_noise = skia_safe::Shader::turbulence_perlin_noise(
            (freq[0] as f32, freq[1] as f32),
            octaves,
            seed as f32,
            Some(skia_safe::ISize::new(
                tile_size[0] as i32,
                tile_size[1] as i32,
            )),
        );
        Ok(Shader {
            label: "turbulence".to_string(),
            shader: shader_turbulence_noise,
        })
    }
    unsafe fn from_png(
        png_bytes: savvy::RawSexp,
        mode: &TileMode,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let input = Data::new_bytes(png_bytes.as_slice());
        let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
            .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;
        Ok(Shader {
            label: "image".to_string(),
            shader: image.to_shader(
                Some((sk_tile_mode(&mode), sk_tile_mode(&mode))),
                skia_safe::SamplingOptions::default(),
                &mat[0],
            ),
        })
    }
    fn from_runtime_effect(
        source: &runtime_effect::RuntimeEffect,
        uniforms: savvy::ListSexp,
    ) -> savvy::Result<Self> {
        let builder = runtime_effect::make_builder(source, &uniforms)?;
        let shader = builder
            .make_shader(&skia_safe::Matrix::default())
            .ok_or_else(|| return savvy_err!("Failed to create runtime shader"))?;
        Ok(Shader {
            label: "runtime_effect".to_string(),
            shader: Some(shader),
        })
    }
    fn from_picture(
        img: savvy::RawSexp,
        mode: &TileMode,
        tile_size: NumericSexp,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("tile_size", 2, tile_size.len())?;

        let tile_size = tile_size.as_slice_f64();
        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let picture = read_picture_bytes(&img)?;
        Ok(Shader {
            label: "picture".to_string(),
            shader: Some(picture.to_shader(
                Some((sk_tile_mode(&mode), sk_tile_mode(&mode))),
                skia_safe::FilterMode::Nearest,
                &mat[0],
                Some(&skia_safe::Rect::new(
                    0.0,
                    0.0,
                    tile_size[0] as f32,
                    tile_size[1] as f32,
                )),
            )),
        })
    }
    fn linear_gradient(
        start: NumericSexp,
        end: NumericSexp,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: &TileMode,
        flags: LogicalSexp,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("start", 2, start.len())?;
        assert_len("end", 2, end.len())?;
        assert_len("from", 4, from.len())?;
        assert_len("to", 4, to.len())?;

        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let start = start.as_slice_f64();
        let end = end.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let flags = flags.to_vec()[0];
        let shader_linear_gradient = skia_safe::Shader::linear_gradient(
            (
                (start[0] as f32, start[1] as f32),
                (end[0] as f32, end[1] as f32),
            ),
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat[0]),
        );
        Ok(Shader {
            label: "linear_gradient".to_string(),
            shader: shader_linear_gradient,
        })
    }
    fn radial_gradient(
        center: NumericSexp,
        radius: NumericScalar,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: &TileMode,
        flags: LogicalSexp,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("center", 2, center.len())?;
        assert_len("from", 4, from.len())?;
        assert_len("to", 4, to.len())?;

        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let center = center.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let radius = radius.as_f64();
        let flags = flags.to_vec()[0];
        let shader_radial_gradient = skia_safe::Shader::radial_gradient(
            (center[0] as f32, center[1] as f32),
            radius as f32,
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat[0]),
        );
        Ok(Shader {
            label: "radial_gradient".to_string(),
            shader: shader_radial_gradient,
        })
    }
    fn conical_gradient(
        start: NumericSexp,
        end: NumericSexp,
        radii: NumericSexp,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: &TileMode,
        flags: LogicalSexp,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("start", 2, start.len())?;
        assert_len("end", 2, end.len())?;
        assert_len("radii", 2, radii.len())?;
        assert_len("from", 4, from.len())?;
        assert_len("to", 4, to.len())?;

        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let start = start.as_slice_f64();
        let end = end.as_slice_f64();
        let radii = radii.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let flags = flags.to_vec()[0];
        let shader_conical_gradient = skia_safe::Shader::two_point_conical_gradient(
            (start[0] as f32, start[1] as f32),
            radii[0] as f32,
            (end[0] as f32, end[1] as f32),
            radii[1] as f32,
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat[0]),
        );
        Ok(Shader {
            label: "conical_gradient".to_string(),
            shader: shader_conical_gradient,
        })
    }
    fn sweep_gradient(
        center: NumericSexp,
        start_angle: NumericScalar,
        end_angle: NumericScalar,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: &TileMode,
        flags: LogicalSexp,
        transform: NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("center", 2, center.len())?;
        assert_len("from", 4, from.len())?;
        assert_len("to", 4, to.len())?;

        let mat =
            as_matrix(&transform).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
        let center = center.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let start = start_angle.as_f64();
        let end = end_angle.as_f64();
        let flags = flags.to_vec()[0];
        let shader_sweep_gradient = skia_safe::Shader::sweep_gradient(
            (center[0] as f32, center[1] as f32),
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            Some((start as f32, end as f32)),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat[0]),
        );
        Ok(Shader {
            label: "sweep_gradient".to_string(),
            shader: shader_sweep_gradient,
        })
    }
}

/// TileMode (0-3)
///
/// `TileMode` determines how the source is tiled for shaders.
/// This is not a paint attribute. To specify `TileMode`, directly pass these pointers to shader functions.
///
/// @details
/// The following `TileMode` are available:
///
/// * `Clamp`
/// * `Repeat`
/// * `Mirror`
/// * `Decal`
///
/// @seealso
/// [TileMode in skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/enum.TileMode.html)
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
