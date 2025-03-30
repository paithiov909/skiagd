use savvy::{savvy, savvy_err, NumericSexp, StringSexp};

/// Returns a skia_safe::Matrix
pub fn as_matrix(mat: &NumericSexp) -> anyhow::Result<skia_safe::Matrix, savvy::Error> {
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

/// Returns default matrix as numerics
///
/// @returns A numeric vector of length 9.
/// @noRd
#[savvy]
fn sk_matrix_default() -> savvy::Result<savvy::Sexp> {
    let matrix = skia_safe::Matrix::default();
    let buffer = vec![
        matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5], matrix[6], matrix[7],
        matrix[8],
    ];
    let mut out = savvy::OwnedRealSexp::new(9)?;
    for (i, v) in buffer.iter().enumerate() {
        out.set_elt(i, *v as f64)?;
    }
    Ok(out.into())
}

/// Transforms SVG paths
///
/// @param svg SVG notations to transform.
/// @param mat Matrix for transforming SVG paths.
/// @returns A character vector.
/// @noRd
#[savvy]
fn sk_path_transform(svg: StringSexp, mat: NumericSexp) -> savvy::Result<savvy::Sexp> {
    let mat = as_matrix(&mat)?;
    let mut out = savvy::OwnedStringSexp::new(svg.len())?;
    for (i, s) in svg.iter().enumerate() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?
            .with_transform(&mat);
        let s = skia_safe::utils::parse_path::to_svg(&path);
        out.set_elt(i, &s)?;
    }
    out.into()
}

/// Interpolates between two SVG paths
///
/// @param value A numeric vector of weights.
/// @param first SVG notation. The second or later elements will be ignored.
/// @param second SVG notation. The second or later elements will be ignored.
/// @returns A character vector.
/// @noRd
#[savvy]
fn sk_path_interpolate(value: NumericSexp, first: StringSexp, second: StringSexp) -> savvy::Result<savvy::Sexp> {
    let first = skia_safe::utils::parse_path::from_svg(first.to_vec()[0])
        .ok_or_else(|| return savvy_err!("Failed to parse first svg"))?;
    let second = skia_safe::utils::parse_path::from_svg(second.to_vec()[0])
        .ok_or_else(|| return savvy_err!("Failed to parse second svg"))?;
    if !first.is_interpolatable(&second) {
        return Err(savvy_err!("Paths are not interpolatable"))
    }
    let mut out = savvy::OwnedStringSexp::new(value.len())?;
    for (i, w) in value.iter_f64().enumerate() {
       let path = first.interpolate(&second, w as f32)
            .ok_or_else(|| return savvy_err!("Failed to interpolate at {}", w))?;
       let s = skia_safe::utils::parse_path::to_svg(&path);
       out.set_elt(i, &s)?;
    }
    out.into()
}

/// Returns minimum and maximum axes values of Point array
///
/// @param svg SVG notations.
/// @returns A numeric vector of length `2*svg.len()`.
/// @noRd
#[savvy]
fn sk_path_size(svg: StringSexp) -> savvy::Result<savvy::Sexp> {
    let mut x: Vec<f32>  = Vec::with_capacity(svg.len());
    let mut y: Vec<f32> = Vec::with_capacity(svg.len());
    for s in svg.iter() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?;
        x.push(path.bounds().height());
        y.push(path.bounds().width());
    }
    let mut out = savvy::OwnedRealSexp::new(2 * svg.len())?;
    for (i, v) in x.iter().enumerate() {
        out.set_elt(i, *v as f64)?;
    }
    for (i, v) in y.iter().enumerate() {
        out.set_elt(i + svg.len(), *v as f64)?;
    }
    Ok(out.into())
}
