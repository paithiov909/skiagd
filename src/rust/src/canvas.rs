use savvy::savvy_err;
use skia_safe::{Picture, PictureRecorder};

/// Returns a skia_safe::Picture
pub unsafe fn read_picture_bytes(bytes: &savvy::RawSexp) -> anyhow::Result<Picture, savvy::Error> {
    let picture = Picture::from_bytes(bytes.as_slice())
        .ok_or_else(|| return savvy_err!("Failed to read picture bytes array"))?;
    Ok(picture)
}

pub struct SkiaCanvas {
    width: i32,
    height: i32,
    recorder: PictureRecorder,
}

impl SkiaCanvas {
    #[allow(unused_mut)]
    pub fn new(width: i32, height: i32) -> SkiaCanvas {
        let mut recorder = skia_safe::PictureRecorder::new();
        SkiaCanvas {
            width,
            height,
            recorder,
        }
    }

    pub fn start_recording(&mut self) -> &skia_safe::Canvas {
        let canvas = self.recorder.begin_recording(
            skia_safe::Rect::from_xywh(0.0, 0.0, self.width as f32, self.height as f32),
            None,
        );
        canvas.clear(skia_safe::Color::TRANSPARENT);
        canvas
    }

    pub fn finish_recording(&mut self) -> anyhow::Result<savvy::OwnedRawSexp, savvy::Error> {
        let picture = self
            .recorder
            .finish_recording_as_picture(Some(&skia_safe::Rect::from_xywh(
                0.0,
                0.0,
                self.width as f32,
                self.height as f32,
            )))
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
