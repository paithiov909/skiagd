mod canvas;
mod paint_attrs;
mod path_transform;
mod runtime_effect;

use canvas::{read_picture_bytes, SkiaCanvas};
use paint_attrs::{font, PaintAttrs};
use path_transform::{as_matrix, as_points};

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
    let mat = as_matrix(&mat)?;
    let size = size.to_vec();
    let dat = as_png_data(size[0], size[1], &picture, &mat)?;
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
    let mat = as_matrix(&mat)?;

    let left_top = left_top.as_slice_f64();
    let input = Data::new_bytes(png_bytes.as_slice());
    let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
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

/// Draws SVG paths
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat1 Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param svg SVG strings to draw.
/// @param width Stroke width.
/// @param color Colors.
/// @param mat2 Matrix for transforming SVG path.
/// @param fill_type FillType.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_path(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat1: NumericSexp,
    props: PaintAttrs,
    svg: StringSexp,
    width: NumericSexp,
    color: NumericSexp,
    mat2: NumericSexp, // transform for svg
    fill_type: &paint_attrs::FillType,
) -> savvy::Result<savvy::Sexp> {
    let width = width.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat1 = as_matrix(&mat1)?;
    let mat2 = as_matrix(&mat2)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat1), Some(&Paint::default()));

    for (i, s) in svg.iter().enumerate() {
        props.reset_color(color[i]);
        props.reset_width(width[i]);
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg at {}", i + 1))?
            .set_fill_type(paint_attrs::sk_fill_type(&fill_type))
            .with_transform(&mat2);
        canvas.draw_path(&path, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws textpaths
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat1 Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param text Text strings to draw along SVG paths.
/// @param svg SVG paths.
/// @param mat2 Matrix for transforming SVG paths.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_textpath(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat1: NumericSexp,
    props: PaintAttrs,
    text: StringSexp,
    svg: StringSexp,
    mat2: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    // https://github.com/Shopify/react-native-skia/blob/main/packages/skia/cpp/api/recorder/Drawings.h#L238
    let typeface = font::match_family_style(props.font_family.as_str(), props.font_face)?;
    let font = skia_safe::Font::from_typeface(&typeface, props.font_size);

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat1 = as_matrix(&mat1)?;
    let mat2 = as_matrix(&mat2)?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat1), Some(&Paint::default()));

    for (i, (t, s)) in text.iter().zip(svg.iter()).enumerate() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg at {}", i + 1))?
            .with_transform(&mat2);
        let ids = font.text_to_glyphs_vec(t.to_string());
        let mut num_ids: Vec<f32> = Vec::new();
        num_ids.resize(font.count_text(t.to_string()), 0.0);
        let width_ptrs = num_ids.as_mut_slice();
        font.get_widths_bounds(ids.as_slice(), Some(width_ptrs), None, Some(&props.paint));

        let mut meas = skia_safe::ContourMeasureIter::from_path(&path, false, Some(1.0));
        let mut dist = 0.0; // initial_offset
        let mut rsx: Vec<skia_safe::RSXform> = Vec::new();
        let mut cont = meas.next().unwrap();

        let mut text_body = t;

        for j in 0..font.count_text(t.to_string()) {
            let width = width_ptrs[j];
            dist += width / 2.0;
            if dist > cont.length() {
                if meas.next().is_none() {
                    text_body = &t[..j];
                    break;
                }
                cont = meas.next().unwrap();
                dist = width / 2.0;
                // FIXME: need to handle when text is longer than path
                // continue;
            }
            let (pos, tan) = cont
                .pos_tan(dist)
                .ok_or_else(|| return savvy_err!("Failed to get pos"))?;
            rsx.push(skia_safe::RSXform {
                scos: tan.x,
                ssin: tan.y,
                tx: pos.x - (width / 2.0) * tan.x,
                ty: pos.y - (width / 2.0) * tan.y,
            });
            dist += width / 2.0;
        }
        let blob = skia_safe::TextBlob::from_rsxform(&text_body, rsx.as_slice(), &font)
            .ok_or_else(|| return savvy_err!("Failed to create text blob at {}", i + 1))?;
        canvas.draw_text_blob(&blob, (0, 0), &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

/// Draws textblobs
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param text Text strings.
/// @param x X coordinates of points where to draw each character.
/// @param y Y coordinates of points where to draw each character.
/// @param color Colors.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_textblob(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    text: StringSexp,
    x: NumericSexp,
    y: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let points = as_points(&x, &y);
    let typeface = font::match_family_style(props.font_family.as_str(), props.font_face)?;
    let font = skia_safe::Font::from_typeface(&typeface, props.font_size);
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(text.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), text.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let mut chars_offset = 0;
    for (i, t) in text.iter().enumerate() {
        let chars = t.to_string();
        let n_chars = font.count_text(&chars);
        let blob = skia_safe::TextBlob::from_pos_text(
            &chars,
            &points[chars_offset..chars_offset + n_chars],
            &font,
        )
        .ok_or_else(|| return savvy_err!("Failed to create text blob at {}", i + 1))?;
        chars_offset += n_chars;
        props.reset_color(color[i]);
        canvas.draw_text_blob(blob, (0, 0), &props.paint);
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
    paint_attrs::assert_len(color.len(), text.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for (i, t) in text.iter().enumerate() {
        let blob = skia_safe::TextBlob::new(t, &font)
        .ok_or_else(|| return savvy_err!("Failed to create text blob at {}", i + 1))?;
        props.reset_color(color[i]);
        canvas.draw_text_blob(&blob, (0.0 as f32, props.font_size), &props.paint);
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
    let mat = as_matrix(&mat)?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

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
    let from_x = from_x.as_slice_f64();
    let from_y = from_y.as_slice_f64();
    let to_x = to_x.as_slice_f64();
    let to_y = to_y.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        canvas.draw_line(
            (from_x[i] as f32, from_y[i] as f32),
            (to_x[i] as f32, to_y[i] as f32),
            &props.paint,
        );
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
    let x = x.as_slice_f64();
    let y = y.as_slice_f64();
    let radius = radius.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        canvas.draw_circle((x[i] as f32, y[i] as f32), radius[i] as f32, &props.paint);
    }
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

// TODO: sk_draw_arc

/// Draws rounded rectangles
///
/// @param size Canvas size.
/// @param curr_bytes Current canvas state.
/// @param mat Matrix for transforming picture.
/// @param props PaintAttrs.
/// @param left X coordinates of the left edge of the rectangles.
/// @param top Y coordinates of the top edge of the rectangles.
/// @param right X coordinates of the right edge of the rectangles.
/// @param bottom Y coordinates of the bottom edge of the rectangles.
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
    left: NumericSexp,
    top: NumericSexp,
    right: NumericSexp,
    bottom: NumericSexp,
    rx: NumericSexp,
    ry: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let left = left.as_slice_f64();
    let top = top.as_slice_f64();
    let right = right.as_slice_f64();
    let bottom = bottom.as_slice_f64();
    let rx = rx.as_slice_f64();
    let ry = ry.as_slice_f64();
    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        let rect = skia_safe::Rect::new(
            left[i] as f32,
            top[i] as f32,
            right[i] as f32,
            bottom[i] as f32,
        );
        canvas.draw_round_rect(&rect, rx[i] as f32, ry[i] as f32, &props.paint);
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
/// @param outer_left X coordinates of the left edge of the outer rectangle.
/// @param outer_top Y coordinates of the top edge of the outer rectangle.
/// @param outer_right X coordinates of the right edge of the outer rectangle.
/// @param outer_bottom Y coordinates of the bottom edge of the outer rectangle.
/// @param outer_rx Axis lengths on X-axis of outer oval describing rounded corners.
/// @param outer_ry Axis lengths on Y-axis of outer oval describing rounded corners.
/// @param inner_left X coordinates of the left edge of the inner rectangle.
/// @param inner_top Y coordinates of the top edge of the inner rectangle.
/// @param inner_right X coordinates of the right edge of the inner rectangle.
/// @param inner_bottom Y coordinates of the bottom edge of the inner rectangle.
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
    outer_left: NumericSexp,
    outer_top: NumericSexp,
    outer_right: NumericSexp,
    outer_bottom: NumericSexp,
    outer_rx: NumericSexp,
    outer_ry: NumericSexp,
    inner_left: NumericSexp,
    inner_top: NumericSexp,
    inner_right: NumericSexp,
    inner_bottom: NumericSexp,
    inner_rx: NumericSexp,
    inner_ry: NumericSexp,
    width: NumericSexp,
    color: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let outer_left = outer_left.as_slice_f64();
    let outer_top = outer_top.as_slice_f64();
    let outer_right = outer_right.as_slice_f64();
    let outer_bottom = outer_bottom.as_slice_f64();
    let outer_rx = outer_rx.as_slice_f64();
    let outer_ry = outer_ry.as_slice_f64();

    let inner_left = inner_left.as_slice_f64();
    let inner_top = inner_top.as_slice_f64();
    let inner_right = inner_right.as_slice_f64();
    let inner_bottom = inner_bottom.as_slice_f64();
    let inner_rx = inner_rx.as_slice_f64();
    let inner_ry = inner_ry.as_slice_f64();

    let color = paint_attrs::num2colors(&color).unwrap_or_else(|| {
        // if matrix is too small to take color, implicitly use paint color
        let mut ret: Vec<skia_safe::Color> = Vec::new();
        ret.resize(width.len(), props.paint.color());
        ret
    });
    paint_attrs::assert_len(color.len(), width.len())?;

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;
    let mut props = props.clone();

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for (i, w) in width.iter_f64().enumerate() {
        props.reset_width(w);
        props.reset_color(color[i]);
        let outer = skia_safe::Rect::new(
            outer_left[i] as f32,
            outer_top[i] as f32,
            outer_right[i] as f32,
            outer_bottom[i] as f32,
        );
        let outer = skia_safe::RRect::new_rect_xy(outer, outer_rx[i] as f32, outer_ry[i] as f32);
        let inner = skia_safe::Rect::new(
            inner_left[i] as f32,
            inner_top[i] as f32,
            inner_right[i] as f32,
            inner_bottom[i] as f32,
        );
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
/// @param scale Scale factor.
/// @param radians Rotation factor.
/// @param tx X translation.
/// @param ty Y translation.
/// @param anchor_x X coordinates of anchor points.
/// @param anchor_y Y coordinates of anchor points.
/// @returns A raw vector of picture.
/// @noRd
#[savvy]
unsafe fn sk_draw_atlas(
    size: IntegerSexp,
    curr_bytes: savvy::RawSexp,
    mat: NumericSexp,
    props: PaintAttrs,
    png_bytes: savvy::RawSexp,
    scale: NumericSexp,
    radians: NumericSexp,
    tx: NumericSexp,
    ty: NumericSexp,
    anchor_x: NumericSexp,
    anchor_y: NumericSexp,
) -> savvy::Result<savvy::Sexp> {
    let input = Data::new_bytes(png_bytes.as_slice());
    let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    let scale = scale.as_slice_f64();
    let radians = radians.as_slice_f64();
    let tx = tx.as_slice_f64();
    let ty = ty.as_slice_f64();
    let anchor_x = anchor_x.as_slice_f64();
    let anchor_y = anchor_y.as_slice_f64();

    let mut transforms: Vec<skia_safe::RSXform> = Vec::new();
    let mut rects: Vec<skia_safe::Rect> = Vec::new();
    for i in 0..scale.len() {
        let trans = skia_safe::RSXform::from_radians(
            scale[i] as f32,
            radians[i] as f32,
            (tx[i] as f32, ty[i] as f32),
            (anchor_x[i] as f32, anchor_y[i] as f32),
        );
        transforms.push(trans);
        rects.push(skia_safe::Rect::new(
            0.0,
            0.0,
            image.width() as f32,
            image.height() as f32,
        ));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

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
    paint_attrs::assert_len(color.len(), positions.len())?;

    let vertices = skia_safe::Vertices::new_copy(mode, &positions, &positions, &color, None);

    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    canvas.draw_vertices(
        &vertices,
        props.paint.blend_mode_or(skia_safe::BlendMode::DstOver),
        &props.paint,
    );
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
    if fill.len() != 4 {
        return Err(savvy_err!("Invalid fill. Expected 4 elements"));
    }
    let fill = fill.as_slice_i32()?;
    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.clear(skia_safe::Color::from_argb(
        fill[3] as u8,
        fill[0] as u8,
        fill[1] as u8,
        fill[2] as u8,
    ));
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}
