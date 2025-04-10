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
