mod canvas;
mod paint_attrs;
mod path_transform;
mod runtime_effect;

use canvas::{read_picture_bytes, SkiaCanvas};
use paint_attrs::{font, PaintAttrs};
use path_transform::{as_matrix, as_points, as_rects};

use savvy::{savvy, savvy_err, IntegerSexp, NumericSexp, StringSexp};
use skia_safe::{Data, Image, Paint};

/// For internal use. See `sk_as_png()`
unsafe fn as_png_data(
    width: i32,
    height: i32,
    picture: &skia_safe::Picture,
    mat: &skia_safe::Matrix,
) -> savvy::Result<Data> {
    let mut surface = skia_safe::surfaces::raster_n32_premul((width, height))
        .unwrap_or_else(|| skia_safe::surfaces::raster_n32_premul((720, 576)).unwrap());
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    surface
        .canvas()
        .draw_picture(&picture, Some(&mat), Some(&Paint::default()));

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
    if size.len() != 2 {
        return Err(savvy_err!("Invalid canvas size. Expected 2 elements"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let size = size.to_vec();
    let dat = as_png_data(size[0], size[1], &picture, &mat[0])?;
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
/// @param props PaintAttrs.
/// @param png_bytes PNG data to draw.
/// @param left_top Offset for drawing PNG image.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_png(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    png_bytes: savvy::RawSexp,
    left_top: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    if left_top.len() != 2 {
        return Err(savvy_err!("Invalid left_top. Expected 2 elements"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;

    let left_top = left_top.as_slice_f64();
    let input = Data::new_bytes(png_bytes.as_slice());
    let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));
    canvas.draw_image(
        &image,
        (left_top[0] as f32, left_top[1] as f32),
        Some(&props.paint),
    );
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws SVG paths
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param svg SVG strings to draw.
/// @param rsx_trans RSX transform for each path.
/// @param width Stroke width.
/// @param color Colors.
/// @param fill_type FillType.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_path(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    svg: StringSexp,
    rsx_trans: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
    fill_type: &paint_attrs::FillType,
) -> savvy::Result<savvy::Sexp> {
    let width = width.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    let transforms = path_transform::as_rsx_trans(&rsx_trans)
        .ok_or_else(|| return savvy_err!("Failed to parse rsx_trans"))?;
    paint_attrs::assert_len("color", color.len(), width.len())?;
    paint_attrs::assert_len("rsx_trans", transforms.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    for (i, s) in svg.iter().enumerate() {
        props.reset_color(color[i]);
        props.reset_width(width[i]);
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg at {}", i + 1))?
            .set_fill_type(paint_attrs::sk_fill_type(&fill_type))
            .with_transform(&skia_safe::Matrix::default().set_rsxform(&transforms[i]));
        canvas.draw_path(&path, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws text as textblobs
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param text Text strings.
/// @param rsx_trans RSX transform for each character.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_text(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    text: StringSexp,
    rsx_trans: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let typeface = font::match_family_style(props.font_family.as_str(), props.font_face)?;
    let font = skia_safe::Font::from_typeface(&typeface, props.font_size);
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(text.len(), props.paint.color());
        ret
    });
    let transforms = path_transform::as_rsx_trans(&rsx_trans)
        .ok_or_else(|| return savvy_err!("Failed to parse rsx_trans"))?;
    paint_attrs::assert_len("color", color.len(), text.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    let mut trans_offset = 0;
    for (i, t) in text.iter().enumerate() {
        let chars = t.to_string();
        let n_chars = font.count_text(&chars);
        if trans_offset + n_chars > transforms.len() {
            return Err(savvy_err!("rsx_trans offset out of bounds"));
        }
        let trans = &transforms[trans_offset..trans_offset + n_chars];
        trans_offset += n_chars;

        let blob = skia_safe::TextBlob::from_rsxform(&chars, trans, &font)
            .ok_or_else(|| return savvy_err!("Failed to create text blob at {}", i + 1))?;

        props.reset_color(color[i]);
        canvas.draw_text_blob(&blob, (0.0, 0.0), &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws points with specified mode
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
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
    props: PaintAttrs,
    x: NumericSexp,
    y: NumericSexp,
    mode: &paint_attrs::PointMode,
) -> savvy::Result<savvy::Sexp> {
    let mode = paint_attrs::sk_point_mode(&mode);
    let points = as_points(&x, &y);

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    canvas.draw_points(mode, &points, &props.paint);
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws lines
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param from_x X coordinates of start points.
/// @param from_y Y coordinates of start points.
/// @param to_x X coordinates of end points.
/// @param to_y Y coordinates of end points.
/// @param width Stroke width.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[allow(unused_mut)]
#[savvy]
unsafe fn sk_draw_line(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    from_x: NumericSexp,
    from_y: NumericSexp,
    to_x: NumericSexp,
    to_y: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let from = as_points(&from_x, &from_y);
    let to = as_points(&to_x, &to_y);
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len("color", color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        canvas.draw_line(from[i], to[i], &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws circles
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param x X coordinates of center.
/// @param y Y coordinates of center.
/// @param radius Circle radius.
/// @param width Stroke width.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_circle(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    x: NumericSexp,
    y: NumericSexp,
    radius: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let center = as_points(&x, &y);
    let radius = radius.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len("color", color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        canvas.draw_circle(center[i], radius[i] as f32, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

// TODO: sk_draw_arc and oval

/// Draws rounded rectangles
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param xywh Rectangles.
/// @param rx Axis lengths on X-axis of oval describing rounded corners.
/// @param ry Axis lengths on Y-axis of oval describing rounded corners.
/// @param width Stroke width.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_rounded_rect(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    xywh: NumericSexp,
    rx: NumericSexp,
    ry: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let rects = as_rects(&xywh).ok_or_else(|| return savvy_err!("Failed to parse xywh"))?;
    let rx = rx.as_slice_f64();
    let ry = ry.as_slice_f64();
    let width = width.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len("color", color.len(), rects.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    for (i, rect) in rects.iter().enumerate() {
        props.reset_width(width[i]);
        props.reset_color(color[i]);
        canvas.draw_round_rect(rect, rx[i] as f32, ry[i] as f32, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws outer and inner rounded rectangles
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param outer_xywh Outer rectangles.
/// @param outer_rx Axis lengths on X-axis of outer oval describing rounded corners.
/// @param outer_ry Axis lengths on Y-axis of outer oval describing rounded corners.
/// @param inner_xywh Inner rectangles.
/// @param inner_rx Axis lengths on X-axis of inner oval describing rounded corners.
/// @param inner_ry Axis lengths on Y-axis of inner oval describing rounded corners.
/// @param width Stroke width.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_diff_rect(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    outer_xywh: NumericSexp,
    outer_rx: NumericSexp,
    outer_ry: NumericSexp,
    inner_xywh: NumericSexp,
    inner_rx: NumericSexp,
    inner_ry: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let outer =
        as_rects(&outer_xywh).ok_or_else(|| return savvy_err!("Failed to parse outer xywh"))?;
    let inner =
        as_rects(&inner_xywh).ok_or_else(|| return savvy_err!("Failed to parse inner xywh"))?;
    let outer_rx = outer_rx.as_slice_f64();
    let outer_ry = outer_ry.as_slice_f64();
    let inner_rx = inner_rx.as_slice_f64();
    let inner_ry = inner_ry.as_slice_f64();
    let width = width.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len("color", color.len(), outer.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    for (i, (outer, inner)) in outer.iter().zip(inner.iter()).enumerate() {
        props.reset_width(width[i]);
        props.reset_color(color[i]);
        let outer = skia_safe::RRect::new_rect_xy(outer, outer_rx[i] as f32, outer_ry[i] as f32);
        let inner = skia_safe::RRect::new_rect_xy(inner, inner_rx[i] as f32, inner_ry[i] as f32);
        canvas.draw_drrect(&outer, &inner, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws atlas
///
/// This function doesn't take `sprites` (offsets for the sprites) argument.
/// The entire image is always used as a sprite.
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param png_bytes PNG bytes.
/// @param rsx_trans RSX transforms for each sprite.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_atlas(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    png_bytes: savvy::RawSexp,
    rsx_trans: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let input = Data::new_bytes(png_bytes.as_slice());
    let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    // Does not need to check the number of rows here.
    let transforms = path_transform::as_rsx_trans(&rsx_trans)
        .ok_or_else(|| return savvy_err!("Failed to parse rsx_trans"))?;
    let mut rects = Vec::new();
    rects.resize(
        transforms.len(),
        skia_safe::Rect::new(0.0, 0.0, image.width() as f32, image.height() as f32),
    );
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    canvas.draw_atlas(
        &image,
        &transforms,
        &rects,
        None,
        props.paint.blend_mode_or(skia_safe::BlendMode::SrcOver),
        skia_safe::SamplingOptions::from_aniso(0), // Aniso level
        None,
        &props.paint,
    );
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws vertices
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param x X coordinates of points.
/// @param y Y coordinates of points.
/// @param color Colors of vertices.
/// @param mode VertexMode.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_vertices(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    x: NumericSexp,
    y: NumericSexp,
    color: NumericSexp,
    mode: &paint_attrs::VertexMode,
) -> savvy::Result<savvy::Sexp> {
    let mode = paint_attrs::sk_vertex_mode(&mode);
    let positions = as_points(&x, &y);
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(positions.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len("color", color.len(), positions.len())?;

    let vertices = skia_safe::Vertices::new_copy(mode, &positions, &positions, &color, None);

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat).ok_or_else(|| return savvy_err!("Failed to parse transform"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&Paint::default()));

    canvas.draw_vertices(
        &vertices,
        props.paint.blend_mode_or(skia_safe::BlendMode::DstOver),
        &props.paint,
    );
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Fills canvas with the specified color
///
/// @param size Canvas size.
/// @param fill Integers of length 4 (RGBA).
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
fn sk_absolute_fill(size: IntegerSexp, fill: NumericSexp) -> savvy::Result<savvy::Sexp> {
    let fill =
        paint_attrs::num2colors(&fill).ok_or_else(|| return savvy_err!("Failed to parse color"))?;
    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.clear(fill[0]);
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}
