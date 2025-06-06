use savvy::{savvy, savvy_err, NumericSexp, StringSexp};

/// Returns Vec<skia_safe::Point>
pub fn as_points(x: &NumericSexp, y: &NumericSexp) -> Vec<skia_safe::Point> {
    let points = std::iter::zip(x.iter_f64(), y.iter_f64())
        .map(|(p0, p1)| skia_safe::Point::new(p0 as f32, p1 as f32))
        .collect::<Vec<skia_safe::Point>>();
    points
}

/// Converts NumericSexp to Vec<skia_safe::RRect>
pub fn as_rrects(
    rect: &NumericSexp,
    rx: &NumericSexp,
    ry: &NumericSexp,
) -> Option<Vec<skia_safe::RRect>> {
    let data = rect.as_slice_f64();
    let rx = rx.as_slice_f64();
    let ry = ry.as_slice_f64();
    let mut ret: Vec<skia_safe::RRect> = Vec::new();
    for (i, chunk) in data.chunks(4).enumerate() {
        if chunk.len() == 4 {
            let out = skia_safe::Rect::new(
                chunk[0] as f32,
                chunk[1] as f32,
                chunk[2] as f32,
                chunk[3] as f32,
            );
            ret.push(skia_safe::RRect::new_rect_xy(
                out,
                rx[i] as f32,
                ry[i] as f32,
            ));
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

/// Converts NumericSexp to Vec<skia_safe::Matrix>
pub fn as_matrix(mat: &NumericSexp) -> Option<Vec<skia_safe::Matrix>> {
    let mat = mat.as_slice_f64();
    let mut ret: Vec<skia_safe::Matrix> = Vec::new();
    for chunk in mat.chunks(9) {
        if chunk.len() == 9 {
            let out = skia_safe::Matrix::new_all(
                chunk[0] as f32,
                chunk[1] as f32,
                chunk[2] as f32,
                chunk[3] as f32,
                chunk[4] as f32,
                chunk[5] as f32,
                chunk[6] as f32,
                chunk[7] as f32,
                chunk[8] as f32,
            );
            ret.push(out);
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

/// Converts NumericSexp to Vec<skia_safe::RSXform>
pub fn as_rsx_trans(rsx_trans: &NumericSexp) -> Option<Vec<skia_safe::RSXform>> {
    let data = rsx_trans.as_slice_f64();
    let mut ret: Vec<skia_safe::RSXform> = Vec::new();
    for chunk in data.chunks(6) {
        if chunk.len() == 6 {
            let scale = chunk[0];
            let radians = chunk[1];
            let tx = chunk[2];
            let ty = chunk[3];
            let anchor_x = chunk[4];
            let anchor_y = chunk[5];
            let trans = skia_safe::RSXform::from_radians(
                scale as f32,
                radians as f32,
                (tx as f32, ty as f32),
                (anchor_x as f32, anchor_y as f32),
            );
            ret.push(trans);
        }
    }
    if ret.is_empty() {
        None
    } else {
        Some(ret)
    }
}

/// Creates a matrix for mapping points
///
/// @param src_x X coordinates of source points.
/// @param src_y Y coordinates of source points.
/// @param dst_x X coordinates of destination points.
/// @param dst_y Y coordinates of destination points.
/// @returns A numeric vector of length 9.
/// @noRd
#[savvy]
fn sk_matrix_map_point(
    src_x: NumericSexp,
    src_y: NumericSexp,
    dst_x: NumericSexp,
    dst_y: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let src = as_points(&src_x, &src_y);
    let dst = as_points(&dst_x, &dst_y);
    let matrix = skia_safe::Matrix::from_poly_to_poly(&src, &dst)
        .ok_or_else(|| return savvy_err!("Failed to map points"))?;
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
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut out = savvy::OwnedStringSexp::new(svg.len())?;
    for (i, s) in svg.iter().enumerate() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg at {}", i + 1))?
            .with_transform(&mat[0]);
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
fn sk_path_interpolate(
    value: NumericSexp,
    first: StringSexp,
    second: StringSexp,
) -> savvy::Result<savvy::Sexp> {
    let first = skia_safe::utils::parse_path::from_svg(first.to_vec()[0])
        .ok_or_else(|| return savvy_err!("Failed to parse first svg"))?;
    let second = skia_safe::utils::parse_path::from_svg(second.to_vec()[0])
        .ok_or_else(|| return savvy_err!("Failed to parse second svg"))?;
    if !first.is_interpolatable(&second) {
        return Err(savvy_err!("Paths are not interpolatable"));
    }
    let mut out = savvy::OwnedStringSexp::new(value.len())?;
    for (i, w) in value.iter_f64().enumerate() {
        let path = first
            .interpolate(&second, w as f32)
            .ok_or_else(|| return savvy_err!("Failed to interpolate for {}", w))?;
        let s = skia_safe::utils::parse_path::to_svg(&path);
        out.set_elt(i, &s)?;
    }
    out.into()
}

/// Returns bounds of SVG paths
///
/// @param svg SVG notations.
/// @returns A list of numeric vectors.
/// @noRd
#[savvy]
fn sk_path_bounds(svg: StringSexp) -> savvy::Result<savvy::Sexp> {
    let mut ids = savvy::OwnedIntegerSexp::new(svg.len())?;
    let mut left = savvy::OwnedRealSexp::new(svg.len())?;
    let mut top = savvy::OwnedRealSexp::new(svg.len())?;
    let mut right = savvy::OwnedRealSexp::new(svg.len())?;
    let mut bottom = savvy::OwnedRealSexp::new(svg.len())?;
    for (i, s) in svg.iter().enumerate() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg at {}", i + 1))?;
        ids.set_elt(i, i as i32)?;
        left.set_elt(i, path.bounds().left() as f64)?;
        top.set_elt(i, path.bounds().top() as f64)?;
        right.set_elt(i, path.bounds().right() as f64)?;
        bottom.set_elt(i, path.bounds().bottom() as f64)?;
    }
    let mut out = savvy::OwnedListSexp::new(5, true)?;
    out.set_name_and_value(0, "id", ids)?;
    out.set_name_and_value(1, "left", left)?;
    out.set_name_and_value(2, "top", top)?;
    out.set_name_and_value(3, "right", right)?;
    out.set_name_and_value(4, "bottom", bottom)?;
    Ok(out.into())
}
