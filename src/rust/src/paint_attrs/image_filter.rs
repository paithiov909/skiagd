use savvy::savvy;

/// @export
#[savvy]
pub struct ImageFilter {
    pub label: String,
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
}
