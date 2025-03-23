mod canvas;
mod matrix;
mod paint_attrs;

use canvas::{read_picture_bytes, SkiaCanvas};
use matrix::as_matrix;
use paint_attrs::shader::{sk_blend_mode, sk_tile_mode, BlendMode, Shader, TileMode};
use paint_attrs::{path_effect::PathEffect, PaintAttrs};

use savvy::{savvy, savvy_err, IntegerSexp, LogicalSexp, NumericScalar, NumericSexp, StringSexp};
use skia_safe::{Data, Image, Paint};

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
/// @param props PaintAttrs.
/// @param svg SVG strings to draw.
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
    mat2: NumericSexp, // transform
    fill_type: paint_attrs::FillType,
) -> savvy::Result<savvy::Sexp> {
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat1 = as_matrix(&mat1)?;
    let mat2 = as_matrix(&mat2)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat1), Some(&Paint::default()));

    for (_, s) in svg.iter().enumerate() {
        let path = skia_safe::utils::parse_path::from_svg(s)
            .ok_or_else(|| return savvy_err!("Failed to parse svg"))?
            .set_fill_type(paint_attrs::sk_fill_type(&fill_type))
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
    mode: paint_attrs::PointMode,
) -> savvy::Result<savvy::Sexp> {
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    let mode = paint_attrs::sk_point_mode(&mode);
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
/// @param props PaintAttrs.
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
    props: PaintAttrs,
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
    props: PaintAttrs,
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

/// Draws rounded rectangle on canvas
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
) -> savvy::Result<savvy::Sexp> {
    if left.len() != top.len()
        || right.len() != top.len()
        || bottom.len() != top.len()
        || rx.len() != top.len()
        || ry.len() != top.len()
    {
        return Err(savvy_err!("All vectors must have the same length"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

    let left = left.as_slice_f64();
    let top = top.as_slice_f64();
    let right = right.as_slice_f64();
    let bottom = bottom.as_slice_f64();
    let rx = rx.as_slice_f64();
    let ry = ry.as_slice_f64();

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for i in 0..left.len() {
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

/// Draws outer and inner rounded rectangles on canvas
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
) -> savvy::Result<savvy::Sexp> {
    if outer_left.len() != outer_top.len()
        || outer_right.len() != outer_top.len()
        || outer_bottom.len() != outer_top.len()
        || outer_rx.len() != outer_top.len()
        || outer_ry.len() != outer_top.len()
        || inner_left.len() != inner_top.len()
        || inner_right.len() != inner_top.len()
        || inner_bottom.len() != inner_top.len()
        || inner_rx.len() != inner_top.len()
        || inner_ry.len() != inner_top.len()
    {
        return Err(savvy_err!("All vectors must have the same length"));
    }
    let picture = read_picture_bytes(&curr_bytes)?;
    let mat = as_matrix(&mat)?;

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

    let size = size.to_vec();
    let mut recorder = SkiaCanvas::new(size[0], size[1]);
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat), Some(&Paint::default()));

    for i in 0..outer_left.len() {
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
    canvas.clear(skia_safe::Color::from_argb(
        fill[3] as u8,
        fill[0] as u8,
        fill[1] as u8,
        fill[2] as u8,
    ));
    let picture = recorder.finish_recording()?;
    Ok(picture.into())
}

#[savvy]
impl PathEffect {
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

#[savvy]
impl Shader {
    fn no_shader() -> savvy::Result<Self> {
        Ok(Shader {
            label: "none".to_string(),
            shader: None,
        })
    }
    pub unsafe fn from_png(
        png_bytes: savvy::RawSexp,
        mode: TileMode,
        mat: NumericSexp,
    ) -> savvy::Result<Self> {
        let mat = as_matrix(&mat)?;
        let input = Data::new_bytes(png_bytes.as_slice());
        let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
            .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;
        Ok(Shader {
            label: "image".to_string(),
            shader: image.to_shader(
                Some((sk_tile_mode(&mode), sk_tile_mode(&mode))),
                skia_safe::SamplingOptions::default(),
                &mat,
            ),
        })
    }
    fn color(rgba: NumericSexp) -> savvy::Result<Self> {
        if rgba.len() != 4 {
            return Err(savvy_err!("Invalid color. Expected 4 elements"));
        }
        let color = rgba.as_slice_f64();
        Ok(Shader {
            label: "color".to_string(),
            shader: Some(skia_safe::shader::shaders::color(
                skia_safe::Color::from_argb(
                    color[3] as u8,
                    color[0] as u8,
                    color[1] as u8,
                    color[2] as u8,
                ),
            )),
        })
    }
    fn blend(mode: BlendMode, dst: Shader, src: Shader) -> savvy::Result<Self> {
        let dst = dst.shader.ok_or(savvy_err!("dst shader is required"))?;
        let src = src.shader.ok_or(savvy_err!("src shader is required"))?;
        let shader_blend = skia_safe::shader::shaders::blend(
            skia_safe::Blender::from(sk_blend_mode(&mode)),
            &dst,
            &src,
        );
        Ok(Shader {
            label: "blend".to_string(),
            shader: Some(shader_blend),
        })
    }
    fn fractal_noise(
        freq: NumericSexp,
        octaves: NumericScalar,
        seed: NumericScalar,
        tile_size: NumericSexp,
    ) -> savvy::Result<Self> {
        if freq.len() != 2 || tile_size.len() != 2 {
            return Err(savvy_err!("Invalid arguments"));
        }
        let freq = freq.as_slice_f64();
        let octaves = octaves.as_usize()?;
        let seed = seed.as_f64();
        let tile_size = tile_size.as_slice_f64();
        let shader_fractal_noise = skia_safe::Shader::fractal_perlin_noise(
            (freq[0] as f32, freq[1] as f32),
            octaves,
            seed as f32,
            Some(skia_safe::ISize::new(
                tile_size[0] as i32,
                tile_size[1] as i32,
            )),
        );
        Ok(Shader {
            label: "fractal_noise".to_string(),
            shader: shader_fractal_noise,
        })
    }
    fn turbulence(
        freq: NumericSexp,
        octaves: NumericScalar,
        seed: NumericScalar,
        tile_size: NumericSexp,
    ) -> savvy::Result<Self> {
        if freq.len() != 2 || tile_size.len() != 2 {
            return Err(savvy_err!("Invalid arguments"));
        }
        let freq = freq.as_slice_f64();
        let octaves = octaves.as_usize()?;
        let seed = seed.as_f64();
        let tile_size = tile_size.as_slice_f64();
        let shader_turbulence_noise = skia_safe::Shader::turbulence_perlin_noise(
            (freq[0] as f32, freq[1] as f32),
            octaves,
            seed as f32,
            Some(skia_safe::ISize::new(
                tile_size[0] as i32,
                tile_size[1] as i32,
            )),
        );
        Ok(Shader {
            label: "turbulence".to_string(),
            shader: shader_turbulence_noise,
        })
    }
    fn linear_gradient(
        start: NumericSexp,
        end: NumericSexp,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: TileMode,
        flags: LogicalSexp,
        mat: NumericSexp,
    ) -> savvy::Result<Self> {
        if start.len() != 2 || end.len() != 2 || from.len() != 4 || to.len() != 4 {
            return Err(savvy_err!("Invalid arguments"));
        }
        let mat = as_matrix(&mat)?;
        let start = start.as_slice_f64();
        let end = end.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let flags = flags.to_vec()[0];
        let shader_linear_gradient = skia_safe::Shader::linear_gradient(
            (
                (start[0] as f32, start[1] as f32),
                (end[0] as f32, end[1] as f32),
            ),
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat),
        );
        Ok(Shader {
            label: "linear_gradient".to_string(),
            shader: shader_linear_gradient,
        })
    }
    fn radial_gradient(
        center: NumericSexp,
        radius: NumericScalar,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: TileMode,
        flags: LogicalSexp,
        mat: NumericSexp,
    ) -> savvy::Result<Self> {
        if center.len() != 2 || from.len() != 4 || to.len() != 4 {
            return Err(savvy_err!("Invalid arguments"));
        }
        let mat = as_matrix(&mat)?;
        let center = center.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let radius = radius.as_f64();
        let flags = flags.to_vec()[0];
        let shader_radial_gradient = skia_safe::Shader::radial_gradient(
            (center[0] as f32, center[1] as f32),
            radius as f32,
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat),
        );
        Ok(Shader {
            label: "radial_gradient".to_string(),
            shader: shader_radial_gradient,
        })
    }
    fn conical_gradient(
        start: NumericSexp,
        end: NumericSexp,
        radii: NumericSexp,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: TileMode,
        flags: LogicalSexp,
        mat: NumericSexp,
    ) -> savvy::Result<Self> {
        if start.len() != 2
            || end.len() != 2
            || radii.len() != 2
            || from.len() != 4
            || to.len() != 4
        {
            return Err(savvy_err!("Invalid arguments"));
        }
        let mat = as_matrix(&mat)?;
        let start = start.as_slice_f64();
        let end = end.as_slice_f64();
        let radii = radii.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let flags = flags.to_vec()[0];
        let shader_conical_gradient = skia_safe::Shader::two_point_conical_gradient(
            (start[0] as f32, start[1] as f32),
            radii[0] as f32,
            (end[0] as f32, end[1] as f32),
            radii[1] as f32,
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat),
        );
        Ok(Shader {
            label: "conical_gradient".to_string(),
            shader: shader_conical_gradient,
        })
    }
    fn sweep_gradient(
        center: NumericSexp,
        start_angle: NumericScalar,
        end_angle: NumericScalar,
        from: NumericSexp,
        to: NumericSexp,
        // pos: NumericSexp,
        mode: TileMode,
        flags: LogicalSexp,
        mat: NumericSexp,
    ) -> savvy::Result<Self> {
        if center.len() != 2 || from.len() != 4 || to.len() != 4 {
            return Err(savvy_err!("Invalid arguments"));
        }
        let mat = as_matrix(&mat)?;
        let center = center.as_slice_f64();
        let from = from.as_slice_f64();
        let to = to.as_slice_f64();
        let start = start_angle.as_f64();
        let end = end_angle.as_f64();
        let flags = flags.to_vec()[0];
        let shader_sweep_gradient = skia_safe::Shader::sweep_gradient(
            (center[0] as f32, center[1] as f32),
            skia_safe::gradient_shader::GradientShaderColors::from(
                [
                    skia_safe::Color::from_argb(
                        from[3] as u8,
                        from[0] as u8,
                        from[1] as u8,
                        from[2] as u8,
                    ),
                    skia_safe::Color::from_argb(to[3] as u8, to[0] as u8, to[1] as u8, to[2] as u8),
                ]
                .as_slice(),
            ),
            None,
            sk_tile_mode(&mode),
            Some((start as f32, end as f32)),
            skia_safe::gradient_shader::Flags::from_bits(flags as u32).or(None),
            Some(&mat),
        );
        Ok(Shader {
            label: "sweep_gradient".to_string(),
            shader: shader_sweep_gradient,
        })
    }
}
