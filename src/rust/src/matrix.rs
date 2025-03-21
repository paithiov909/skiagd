use savvy::{savvy, savvy_err, NumericSexp};

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
