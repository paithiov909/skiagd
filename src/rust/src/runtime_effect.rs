use savvy::{savvy, savvy_err};

/// @export
#[savvy]
struct RuntimeEffect {
    pub runtime_effect: skia_safe::RuntimeEffect,
}

#[savvy]
impl RuntimeEffect {
    fn make(sksl: savvy::StringSexp) -> savvy::Result<Self> {
        let sksl = sksl.to_vec()[0];
        // NOTE: This does not work with v0.84.0 above
        let effect = skia_safe::RuntimeEffect::make_for_shader(sksl, None)
            .map_err(|e| savvy_err!("Failed to create RuntimeEffect:\n {}", e))?;
        Ok(RuntimeEffect {
            runtime_effect: effect,
        })
    }
    fn source(&self) -> savvy::Result<savvy::Sexp> {
        let source = self.runtime_effect.source();
        let out = savvy::OwnedStringSexp::try_from_scalar(&source)?;
        Ok(out.into())
    }
}

pub fn make_builder(
    source: &RuntimeEffect,
    uniforms: &savvy::ListSexp,
) -> anyhow::Result<skia_safe::runtime_effect::RuntimeShaderBuilder, savvy::Error> {
    let effect = source.runtime_effect.clone();
    let mut builder = skia_safe::runtime_effect::RuntimeShaderBuilder::new(effect);

    if !uniforms.is_empty() {
        for (n, v) in uniforms.iter() {
            match v.into_typed() {
                savvy::TypedSexp::Real(j) => {
                    let j = j.iter().map(|x| *x as f32).collect::<Vec<f32>>();
                    builder
                        .set_uniform_float(n, &j)
                        .or_else(|_| return Err(savvy_err!("Failed to set uniform: {}", n)))?;
                }
                savvy::TypedSexp::Integer(j) => {
                    builder
                        .set_uniform_int(n, j.as_slice())
                        .or_else(|_| return Err(savvy_err!("Failed to set uniform: {}", n)))?;
                }
                _ => Err(savvy_err!("Invalid type for uniform {}", n))?,
            }
        }
    }
    Ok(builder)
}
