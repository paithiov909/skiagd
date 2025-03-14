mod canvas;
mod paint_props;

use canvas::{as_matrix, read_picture_bytes, SkiaCanvas};
use paint_props::PaintProps;

use savvy::{savvy, savvy_err};
use savvy::{IntegerSexp, NumericSexp, StringSexp};
use skia_safe::{Data, Image, Paint};

/// Returns default matrix as numerics
///
/// @details
/// Users should not touch matrix to transform pictures.
/// For a `canvas.draw_picture()` call, pass `Paint::default()`.
///
/// @returns A numeric vector of length 9.
/// @noRd
#[savvy]
fn sk_matrix_default() -> savvy::Result<savvy::Sexp> {
    let matrix = skia_safe::Matrix::default();
    let buffer = vec![
        matrix[0], matrix[1], matrix[2],
        matrix[3], matrix[4], matrix[5],
        matrix[6], matrix[7], matrix[8]
    ];
    let mut out = savvy::OwnedRealSexp::new(9)?;
    for (i, v) in buffer.iter().enumerate() {
        out.set_elt(i, *v as f64)?;
    }
    Ok(out.into())
}

/// For internal use. See `sk_as_png()`
unsafe fn sk_as_png_data(
    size: &IntegerSexp,
    curr_bytes: &savvy::RawSexp,
    mat: &NumericSexp,
) -> savvy::Result<Data> {
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let size = size.to_vec();
    let mut surface = skia_safe::surfaces::raster_n32_premul((size[0], size[1]))
        .unwrap_or_else(|| skia_safe::surfaces::raster_n32_premul((720, 576)).unwrap());
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    surface.canvas().draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let dat = image
        .encode(context.as_mut(), skia_safe::EncodedImageFormat::PNG, None)
        .unwrap();

    Ok(dat)
}

/// Takes a raw vector of picture and returns PNG data
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @returns A raw vector of PNG data.
/// @noRd
#[savvy]
unsafe fn sk_as_png(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let dat = sk_as_png_data(&size, &curr_bytes, &mat)?;
    let mut ret = savvy::OwnedRawSexp::new(dat.len())?;
    for (i, b) in dat.as_bytes().iter().enumerate() {
        ret.set_elt(i, *b)?;
    }
    Ok(ret.into())
}

/// Draws PNG data as an image on canvas
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintProps.
/// @param png_bytes PNG data to draw.
/// @param left_top Offset for drawing PNG image.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_png(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintProps,
    png_bytes: savvy::RawSexp,
    left_top: NumericSexp, // FIXME: Should be Numeric
) -> savvy::Result<savvy::Sexp> {
    if left_top.len() != 2 {
        return Err(savvy_err!("Invalid left_top. Expected 2 elements"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let left_top = left_top.as_slice_f64();
    let input = Data::new_bytes(png_bytes.as_slice());
    let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));
    canvas.draw_image(
        &image,
        (left_top[0] as f32, left_top[1] as f32),
        Some(&props.paint),
    );
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws SVG path on canvas
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat1 Matrix for transforming picture.
/// @param props PaintProps.
/// @param svg SVG strings to draw.
/// @param trim Numerics of length 2 to trim the start and end of the path.
/// Values are in the range `[0, 1]`.
/// @param mat2 Matrix for transforming SVG path.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_path(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat1: NumericSexp,
    props: PaintProps,
    svg: StringSexp,
    mat2: NumericSexp, // transform
) -> savvy::Result<savvy::Sexp> {
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat1 = as_matrix(&mat1)?;
    let mat2 = as_matrix(&mat2)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat1), Some(&Paint::default()));

    for (_, s) in svg.iter().enumerate() {
        // TODO: set_fill_type
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?
            .with_transform(&mat2);
        canvas.draw_path(&path, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws points with specified mode
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param x X coordinates of points.
/// @param y Y coordinates of points.
/// @param mode PointMode.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_points(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintProps,
    x: NumericSexp,
    y: NumericSexp,
    mode: paint_props::PointMode,
) -> savvy::Result<savvy::Sexp> {
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let mode = paint_props::sk_point_mode(&mode);
    let points = std::iter::zip(x.iter_f64(), y.iter_f64())
        .map(|p| skia_safe::Point::new(p.0 as f32, p.1 as f32))
        .collect::<Vec<skia_safe::Point>>();
    canvas.draw_points(mode, &points, &props.paint);
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws lines on canvas
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintProps.
/// @param from_x X coordinates of start points.
/// @param from_y Y coordinates of start points.
/// @param to_x X coordinates of end points.
/// @param to_y Y coordinates of end points.
/// @returns A raw vector of picture.
/// @noRd
#[allow(unused_mut)]
#[savvy]
unsafe fn sk_draw_line(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintProps,
    from_x: NumericSexp,
    from_y: NumericSexp,
    to_x: NumericSexp,
    to_y: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    if from_x.len() != from_y.len() || to_x.len() != to_y.len() || from_x.len() != to_x.len() {
        return Err(savvy_err!("All vectors must have the same length"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let from_x = from_x.as_slice_f64();
    let from_y = from_y.as_slice_f64();
    let to_x = to_x.as_slice_f64();
    let to_y = to_y.as_slice_f64();
    for i in 0..from_x.len() {
        canvas.draw_line(
            (from_x[i] as f32, from_y[i] as f32),
            (to_x[i] as f32, to_y[i] as f32),
            &props.paint,
        );
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws circle on canvas
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param x X coordinates of center.
/// @param y Y coordinates of center.
/// @param radius Circle radius.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_circle(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintProps,
    x: NumericSexp,
    y: NumericSexp,
    radius: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    if x.len() != radius.len() || y.len() != radius.len() {
        return Err(savvy_err!("Invalid center or radius. Expected same length"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let x = x.as_slice_f64();
    let y = y.as_slice_f64();
    for (i, radii) in radius.iter_f64().enumerate() {
        canvas.draw_circle((x[i] as f32, y[i] as f32), radii as f32, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws rectangle on canvas
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintProps.
/// @param left X coordinates of the left edge of the rectangles.
/// @param top Y coordinates of the top edge of the rectangles.
/// @param right X coordinates of the right edge of the rectangles.
/// @param bottom Y coordinates of the bottom edge of the rectangles.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_irect(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintProps,
    left: NumericSexp,
    top: NumericSexp,
    right: NumericSexp,
    bottom: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    if left.len() != top.len() || right.len() != top.len() || bottom.len() != top.len() {
        return Err(savvy_err!("All vectors must have the same length"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let left = left.as_slice_i32()?;
    let top = top.as_slice_i32()?;
    let right = right.as_slice_i32()?;
    let bottom = bottom.as_slice_i32()?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for i in 0..left.len() {
        let rect = skia_safe::IRect::new(left[i], top[i], right[i], bottom[i]);
        canvas.draw_irect(&rect, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Fills canvas with color
///
/// @param size Canvas size.
/// @param fill Integers of length 4 (RGBA).
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
fn sk_absolute_fill(size: IntegerSexp, fill: NumericSexp) -> savvy::Result<savvy::Sexp> {
    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    let fill = fill.as_slice_i32()?;
    if fill.len() != 4 {
        return Err(savvy_err!("Invalid fill. Expected 4 elements"));
    }
    canvas.clear(
        skia_safe::Color::from_argb(fill[3] as u8, fill[0] as u8, fill[1] as u8, fill[2] as u8)
    );
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}
