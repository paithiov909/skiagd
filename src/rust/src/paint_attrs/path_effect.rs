use savvy::{savvy, savvy_err, NumericScalar, NumericSexp, StringSexp};

// Copied from canvas::as_matrix
fn as_matrix(mat: &NumericSexp) -> anyhow::Result<skia_safe::Matrix, savvy::Error> {
    if mat.len() != 9 {
        return Err(savvy_err!("Invalid matrix. Expected 9 elements"));
    }
    let mat = mat.as_slice_f64();
    let out = skia_safe::Matrix::new_all(
        mat[0] as f32,
        mat[1] as f32,
        mat[2] as f32,
        mat[3] as f32,
        mat[4] as f32,
        mat[5] as f32,
        mat[6] as f32,
        mat[7] as f32,
        mat[8] as f32,
    );
    Ok(out)
}

/// PathEffect
///
/// `PathEffect` is a struct that offers a reference to `skia_safe::PathEffect`.
/// You can apply a path effect to shapes via [paint()].
/// Currently only single `PathEffect` can be specified; multiple effects are not supported.
///
/// @details
/// The following effects are available:
///
/// * `no_effect()`: does not apply any path effect. This is the default effect for `paint()`.
/// * `trim(start, end)`: trims the `start` and `end` of the path. `start` and `end` are in the range `[0, 1]`.
/// * `discrete(length, deviation, seed)`: applies discrete path effect.
/// * `dash(intervals, phase)`: applies dash path effect.
/// * `corner(radius)`: applies corner path effect.
/// * `path_1d(path, advance, phase, style)`: applies 1D path effect. `style` can be `"translate"`, `"rotate"`, or `"morph"`.
/// * `path_2d(path, mat)`: applies 2D path effect.
/// * `line_2d(width, mat)`: applies 2D line path effect.
///
/// @seealso
/// [Path Effects | React Native Skia](https://shopify.github.io/react-native-skia/docs/path-effects/)
/// @export
#[savvy]
pub struct PathEffect {
    label: String,
    pub effect: Option<skia_safe::PathEffect>,
}

#[savvy]
impl PathEffect {
    fn get_label(&self) -> savvy::Result<savvy::Sexp> {
        let label = &self.label;
        let out = savvy::OwnedStringSexp::try_from_scalar(&label)?;
        Ok(out.into())
    }
    fn no_effect() -> savvy::Result<Self> {
        Ok(PathEffect {
            label: "none".to_string(),
            effect: None,
        })
    }
    fn trim(start: NumericScalar, end: NumericScalar) -> savvy::Result<Self> {
        let start = start.as_f64();
        let end = end.as_f64();
        if start < 0.0 || start > 1.0 || end < 0.0 || end > 1.0 {
            return Err(savvy_err!("Invalid trim values"));
        }
        let effect_trim = skia_safe::PathEffect::trim(
            start as f32,
            end as f32,
            skia_safe::trim_path_effect::Mode::Normal,
        )
        .ok_or_else(|| return savvy_err!("Failed to create path effect"))?;
        Ok(PathEffect {
            label: "trim".to_string(),
            effect: Some(effect_trim),
        })
    }
    fn discrete(
        length: NumericScalar,
        deviation: NumericScalar,
        seed: NumericScalar, // must be an integer
    ) -> savvy::Result<Self> {
        let length = length.as_f64();
        let deviation = deviation.as_f64();
        let seed = seed.as_i32()?;
        let effect_discrete =
            skia_safe::PathEffect::discrete(length as f32, deviation as f32, seed as u32);
        Ok(PathEffect {
            label: "discrete".to_string(),
            effect: effect_discrete,
        })
    }
    fn dash(intervals: NumericSexp, phase: NumericScalar) -> savvy::Result<Self> {
        let intervals = intervals.iter_f64().map(|x| x as f32).collect::<Vec<f32>>();
        let phase = phase.as_f64();
        let effect_dash = skia_safe::PathEffect::dash(intervals.as_slice(), phase as f32);
        Ok(PathEffect {
            label: "dash".to_string(),
            effect: effect_dash,
        })
    }
    fn corner(radius: NumericScalar) -> savvy::Result<Self> {
        let radius = radius.as_f64();
        let effect_corner = skia_safe::PathEffect::corner_path(radius as f32);
        Ok(PathEffect {
            label: "corner".to_string(),
            effect: effect_corner,
        })
    }
    fn path_1d(
        path: StringSexp,
        advance: NumericScalar,
        phase: NumericScalar,
        style: StringSexp,
    ) -> savvy::Result<Self> {
        let s = path.to_vec()[0];
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?;
        let style = match style.to_vec()[0] {
            "translate" => skia_safe::path_1d_path_effect::Style::Translate,
            "rotate" => skia_safe::path_1d_path_effect::Style::Rotate,
            "morph" => skia_safe::path_1d_path_effect::Style::Morph,
            _ => skia_safe::path_1d_path_effect::Style::Translate,
        };
        let effect_1d = skia_safe::PathEffect::path_1d(
            &path,
            advance.as_f64() as f32,
            phase.as_f64() as f32,
            style,
        )
        .ok_or_else(|| return savvy_err!("Failed to create path effect"))?;
        Ok(PathEffect {
            label: "path_1d".to_string(),
            effect: Some(effect_1d),
        })
    }
    fn path_2d(path: StringSexp, mat: NumericSexp) -> savvy::Result<Self> {
        let mat = as_matrix(&mat)?;
        let s = path.to_vec()[0];
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?;
        let effect_2d = skia_safe::PathEffect::path_2d(&mat, &path);
        Ok(PathEffect {
            label: "path_2d".to_string(),
            effect: Some(effect_2d),
        })
    }
    fn line_2d(width: NumericScalar, mat: NumericSexp) -> savvy::Result<Self> {
        let mat = as_matrix(&mat)?;
        let effect_2d = skia_safe::PathEffect::line_2d(width.as_f64() as f32, &mat);
        Ok(PathEffect {
            label: "line_2d".to_string(),
            effect: effect_2d,
        })
    }
}
