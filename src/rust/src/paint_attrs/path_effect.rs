use savvy::savvy;

/// @export
#[savvy]
pub struct PathEffect {
    pub label: String,
    pub effect: Option<skia_safe::PathEffect>,
}

#[savvy]
impl PathEffect {
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
}
