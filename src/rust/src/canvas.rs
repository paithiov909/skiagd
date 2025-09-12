use crate::paint_attrs::{assert_len, PaintAttrs};

use savvy::savvy_err;
use skia_safe::{Picture, PictureRecorder};

/// Returns a skia_safe::Picture
pub fn read_picture_bytes(bytes: &savvy::RawSexp) -> anyhow::Result<Picture, savvy::Error> {
    let picture = Picture::from_bytes(bytes.as_slice())
        .ok_or_else(|| return savvy_err!("Failed to read picture bytes array"))?;
    Ok(picture)
}

pub struct SkiaCanvas {
    width: i32,
    height: i32,
    pub recorder: PictureRecorder,
}

impl SkiaCanvas {
    #[allow(unused_mut)]
    pub fn setup(size: &savvy::IntegerSexp) -> anyhow::Result<SkiaCanvas, savvy::Error> {
        assert_len("size", 2, size.len())?;

        let size = size.as_slice();
        let mut recorder = skia_safe::PictureRecorder::new();
        Ok(SkiaCanvas {
            width: size[0],
            height: size[1],
            recorder,
        })
    }

    pub fn start_recording(&mut self) -> &skia_safe::Canvas {
        let canvas = self
            .recorder
            .begin_recording(skia_safe::Rect::from_isize((self.width, self.height)), false);
        canvas.clear(skia_safe::Color::TRANSPARENT);
        canvas
    }

    pub fn finish_recording(&mut self) -> anyhow::Result<savvy::OwnedRawSexp, savvy::Error> {
        let picture = self
            .recorder
            .finish_recording_as_picture(None)
            .ok_or_else(|| return savvy_err!("Failed to finish recording"))?;
        let d = picture.serialize();
        let bytes = d.as_bytes();
        let mut ret = savvy::OwnedRawSexp::new(bytes.len())?;
        for (i, b) in bytes.iter().enumerate() {
            ret.set_elt(i, *b)?;
        }
        Ok(ret)
    }
}

/// Encodes a skia_safe::Picture into PNG, returning a Option<skia_safe::Data>
pub fn as_png(
    size: Vec<i32>,
    picture: skia_safe::Picture,
    mat: &Vec<skia_safe::Matrix>,
) -> Option<skia_safe::Data> {
    let mut surface = skia_safe::surfaces::raster_n32_premul((size[0], size[1]))
        .unwrap_or_else(|| skia_safe::surfaces::raster_n32_premul((720, 576)).unwrap());
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    surface
        .canvas()
        .draw_picture(&picture, Some(&mat[0]), Some(&skia_safe::Paint::default()));

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image.encode(context.as_mut(), skia_safe::EncodedImageFormat::PNG, None);

    data
}

/// Takes PNG as input and puts it onto a canvas
pub fn put_png(
    input: skia_safe::Data,
    size: savvy::IntegerSexp,
    picture: skia_safe::Picture,
    mat: Vec<skia_safe::Matrix>,
    left_top: Vec<f64>,
    props: PaintAttrs,
) -> anyhow::Result<savvy::OwnedRawSexp, savvy::Error> {
    let image = skia_safe::Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
        .ok_or_else(|| return savvy_err!("Failed to read PNG as image"))?;

    let mut recorder = SkiaCanvas::setup(&size)?;
    let canvas = recorder.start_recording();
    canvas.draw_picture(&picture, Some(&mat[0]), Some(&skia_safe::Paint::default()));
    canvas.draw_image(
        &image,
        (left_top[0] as f32, left_top[1] as f32),
        Some(&props.paint),
    );
    let picture = recorder.finish_recording()?;

    Ok(picture)
}
