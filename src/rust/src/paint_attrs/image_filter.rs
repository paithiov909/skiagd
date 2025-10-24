use super::{assert_len, shader::sk_blend_mode, shader::BlendMode};
use crate::canvas::read_picture_bytes;
use crate::runtime_effect;
use savvy::{savvy, savvy_err};

/// @export
#[savvy]
pub struct ImageFilter {
    label: String,
    pub filter: Option<skia_safe::ImageFilter>,
}

#[savvy]
impl ImageFilter {
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
    fn no_filter() -> savvy::Result<Self> {
        Ok(ImageFilter {
            label: "none".to_string(),
            filter: None,
        })
    }
    fn from_picture(img: savvy::RawSexp, crop_rect: savvy::NumericSexp) -> savvy::Result<Self> {
        assert_len("crop_rect", 4, crop_rect.len())?;
        let picture = read_picture_bytes(&img)?;
        let crop_rect = crop_rect.as_slice_f64();
        Ok(ImageFilter {
            label: "picture".to_string(),
            filter: skia_safe::image_filters::picture(
                picture,
                Some(&skia_safe::Rect::new(
                    crop_rect[0] as f32,
                    crop_rect[1] as f32,
                    crop_rect[2] as f32,
                    crop_rect[3] as f32,
                )),
            ),
        })
    }
    fn compose(outer: &ImageFilter, inner: &ImageFilter) -> savvy::Result<Self> {
        let outer = outer
            .filter
            .clone()
            .unwrap_or_else(|| skia_safe::image_filters::empty());
        let inner = inner
            .filter
            .clone()
            .unwrap_or_else(|| skia_safe::image_filters::empty());
        Ok(ImageFilter {
            label: "compose".to_string(),
            filter: skia_safe::image_filters::compose(outer, inner),
        })
    }
    fn blend(
        dst: &ImageFilter,
        src: &ImageFilter,
        mode: &BlendMode,
        crop_rect: savvy::NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("crop_rect", 4, crop_rect.len())?;
        let blend_mode = sk_blend_mode(mode);
        let blender = skia_safe::Blender::mode(blend_mode);
        let crop_rect = crop_rect.as_slice_f64();
        Ok(ImageFilter {
            label: "blend".to_string(),
            filter: skia_safe::image_filters::blend(
                blender,
                dst.filter.clone(),
                src.filter.clone(),
                skia_safe::Rect::new(
                    crop_rect[0] as f32,
                    crop_rect[1] as f32,
                    crop_rect[2] as f32,
                    crop_rect[3] as f32,
                ),
            ),
        })
    }
    fn arithmetic(
        dst: &ImageFilter,
        src: &ImageFilter,
        coef: savvy::NumericSexp,
        crop_rect: savvy::NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("coef", 4, coef.len())?;
        assert_len("crop_rect", 4, crop_rect.len())?;
        let coef = coef.as_slice_f64();
        let crop_rect = crop_rect.as_slice_f64();
        Ok(ImageFilter {
            label: "arithmetic".to_string(),
            filter: skia_safe::image_filters::arithmetic(
                coef[0] as f32,
                coef[1] as f32,
                coef[2] as f32,
                coef[3] as f32,
                true,
                dst.filter.clone(),
                src.filter.clone(),
                skia_safe::Rect::new(
                    crop_rect[0] as f32,
                    crop_rect[1] as f32,
                    crop_rect[2] as f32,
                    crop_rect[3] as f32,
                ),
            ),
        })
    }
    fn color_matrix(color_mat: savvy::NumericSexp) -> savvy::Result<Self> {
        assert_len("color_mat", 20, color_mat.len())?;
        let color_mat = color_mat.as_slice_f64();
        let mat = skia_safe::ColorMatrix::new(
            color_mat[0] as f32,
            color_mat[1] as f32,
            color_mat[2] as f32,
            color_mat[3] as f32,
            color_mat[4] as f32,
            color_mat[5] as f32,
            color_mat[6] as f32,
            color_mat[7] as f32,
            color_mat[8] as f32,
            color_mat[9] as f32,
            color_mat[10] as f32,
            color_mat[11] as f32,
            color_mat[12] as f32,
            color_mat[13] as f32,
            color_mat[14] as f32,
            color_mat[15] as f32,
            color_mat[16] as f32,
            color_mat[17] as f32,
            color_mat[18] as f32,
            color_mat[19] as f32,
        );
        let imgf_color =
            skia_safe::color_filters::matrix(&mat, skia_safe::color_filters::Clamp::Yes);
        Ok(ImageFilter {
            label: "color_filter".to_string(),
            filter: skia_safe::image_filters::color_filter(imgf_color, None, None),
        })
    }
    fn displacement_map(
        channels: savvy::NumericSexp,
        scale: savvy::NumericScalar,
        displacement: &ImageFilter,
        crop_rect: savvy::NumericSexp,
    ) -> savvy::Result<Self> {
        assert_len("channels", 2, channels.len())?;
        assert_len("crop_rect", 4, crop_rect.len())?;
        let channels = channels.as_slice_i32()?;
        let scale = scale.as_f64();
        let crop_rect = crop_rect.as_slice_f64();
        Ok(ImageFilter {
            label: "displacement_map".to_string(),
            filter: skia_safe::image_filters::displacement_map(
                (sk_colorchannel(channels[0]), sk_colorchannel(channels[1])),
                scale as f32,
                displacement.filter.clone(), // displacement
                None,                        // color
                skia_safe::Rect::new(
                    crop_rect[0] as f32,
                    crop_rect[1] as f32,
                    crop_rect[2] as f32,
                    crop_rect[3] as f32,
                ),
            ),
        })
    }
    fn runtime_shader(
        source: &runtime_effect::RuntimeEffect,
        uniforms: savvy::ListSexp,
    ) -> savvy::Result<Self> {
        let builder = runtime_effect::make_builder(source, &uniforms)?;
        let imgf =
            skia_safe::image_filters::runtime_shader(&builder, "", None).ok_or_else(|| {
                return savvy_err!(
                    "Failed to create runtime shader. Maybe the types of uniforms are mismatched"
                );
            })?;
        Ok(ImageFilter {
            label: "runtime_effect".to_string(),
            filter: Some(imgf),
        })
    }
}

fn sk_colorchannel(ch: i32) -> skia_safe::ColorChannel {
    match ch {
        0 => skia_safe::ColorChannel::R,
        1 => skia_safe::ColorChannel::G,
        2 => skia_safe::ColorChannel::B,
        3 => skia_safe::ColorChannel::A,
        _ => skia_safe::ColorChannel::A,
    }
}
