use std::fs::File;
use std::io::Write;

use cxx::CxxVector;

use skia_safe::{surfaces, AlphaType, Color, Data, EncodedImageFormat, Image, Paint, Surface};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Canvas;
        fn skia_canvas(width: i32, height: i32) -> Box<Canvas>;
        unsafe fn read_bytes(&mut self, bytes: &CxxVector<u8>, left: i32, top: i32) -> Result<()>;
        fn save_bytes(&mut self) -> Vec<u8>;
        fn save_png(&mut self, filename: String) -> Result<()>;
        fn new_page(&mut self, col: u32);
        fn circle(&mut self, x: f32, y: f32, r: f32);
        fn set_paint_props(
            &mut self,
            col: u32,
            fill: u32,
            ljoin: u32,
            lend: u32,
            lty: u32,
            lwd: f32,
            lmiter: f32,
            blend_mode: u32,
        );
        fn test(&mut self) -> Result<()>;
    }
}

fn skia_canvas(width: i32, height: i32) -> Box<Canvas> {
    Box::new(Canvas::new(width, height))
}

struct Canvas {
    // filename: String,
    surface: Surface,
    // path: Path,
    paint: Paint,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = surfaces::raster_n32_premul((width, height))
            .unwrap_or_else(|| surfaces::raster_n32_premul((720, 576)).unwrap());
        // let path = Path::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        surface.canvas().clear(Color::TRANSPARENT);
        Canvas {
            surface,
            // path,
            paint,
        }
    }

    #[inline]
    pub unsafe fn read_bytes(
        &mut self,
        bytes: &CxxVector<u8>,
        left: i32,
        top: i32,
    ) -> anyhow::Result<()> {
        let input = Data::new_bytes(bytes.as_slice());
        let image = Image::from_encoded_with_alpha_type(input, AlphaType::Premul)
            .ok_or_else(|| return anyhow::Error::msg("Bomb! failed to read bytes"))?;
        self.surface
            .canvas()
            .clear(Color::TRANSPARENT)
            .draw_image(&image, (left, top), None);
        Ok(())
    }

    #[inline]
    pub fn save_bytes(&mut self) -> Vec<u8> {
        let d = self.data();
        let bytes = d.as_bytes();
        let mut ret = Vec::new();
        ret.extend_from_slice(bytes);
        ret
    }

    pub fn save_png(&mut self, filename: String) -> anyhow::Result<()> {
        let d = self.data();
        let mut file = File::create(filename)?;
        let bytes = d.as_bytes();
        file.write_all(bytes)?;
        Ok(())
    }

    pub fn new_page(&mut self, fill: u32) {
        let [r, g, b, a] = fill.to_ne_bytes();
        self.surface.canvas().clear(Color::from_argb(a, r, g, b));
    }

    pub fn circle(&mut self, x: f32, y: f32, r: f32) {
        self.surface.canvas().draw_circle((x, y), r, &self.paint);
    }

    #[allow(dead_code)]
    fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    #[allow(dead_code)]
    fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    #[inline]
    fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        let mut context = self.surface.direct_context();
        let ret = image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .unwrap();
        ret
    }

    #[inline]
    fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }

    pub fn test(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    #[inline]
    fn fill_to_style(fill: u32) -> skia_safe::PaintStyle {
        // TODO: support `Fill`
        if fill == 0 {
            skia_safe::PaintStyle::Stroke
        } else {
            skia_safe::PaintStyle::StrokeAndFill
        }
    }
    #[inline]
    fn ljoin_to_join(ljoin: u32) -> skia_safe::paint::Join {
        match ljoin {
            1 => skia_safe::paint::Join::Round,
            2 => skia_safe::paint::Join::Miter,
            3 => skia_safe::paint::Join::Bevel,
            _ => skia_safe::paint::Join::Default,
        }
    }
    #[inline]
    fn lend_to_cap(lend: u32) -> skia_safe::PaintCap {
        match lend {
            1 => skia_safe::PaintCap::Round,
            2 => skia_safe::PaintCap::Butt,
            3 => skia_safe::PaintCap::Square,
            _ => skia_safe::PaintCap::Default,
        }
    }

    #[inline]
    fn blend_mode(mode: u32) -> skia_safe::BlendMode {
        match mode {
            0 => skia_safe::BlendMode::Clear,
            1 => skia_safe::BlendMode::Src,
            2 => skia_safe::BlendMode::Dst,
            3 => skia_safe::BlendMode::SrcOver,
            4 => skia_safe::BlendMode::DstOver,
            5 => skia_safe::BlendMode::SrcIn,
            6 => skia_safe::BlendMode::DstIn,
            7 => skia_safe::BlendMode::SrcOut,
            8 => skia_safe::BlendMode::DstOut,
            9 => skia_safe::BlendMode::SrcATop,
            10 => skia_safe::BlendMode::DstATop,
            11 => skia_safe::BlendMode::Xor,
            12 => skia_safe::BlendMode::Modulate,
            13 => skia_safe::BlendMode::Screen,
            14 => skia_safe::BlendMode::Overlay,
            15 => skia_safe::BlendMode::Darken,
            16 => skia_safe::BlendMode::Lighten,
            17 => skia_safe::BlendMode::ColorDodge,
            18 => skia_safe::BlendMode::ColorBurn,
            19 => skia_safe::BlendMode::SoftLight,
            20 => skia_safe::BlendMode::HardLight,
            21 => skia_safe::BlendMode::Difference,
            22 => skia_safe::BlendMode::Exclusion,
            23 => skia_safe::BlendMode::Multiply,
            24 => skia_safe::BlendMode::Hue,
            25 => skia_safe::BlendMode::Saturation,
            26 => skia_safe::BlendMode::Color,
            27 => skia_safe::BlendMode::Luminosity,
            _ => skia_safe::BlendMode::Src,
        }
    }

    // PathEffect
    // MaskFilter
    // ImageFilter
    // ColorFilter
    // Shader

    #[allow(unused_variables)]
    #[inline]
    pub fn set_paint_props(
        &mut self,
        col: u32,
        fill: u32,
        ljoin: u32,  // stroke join (bevel, miter, round)
        lend: u32,   // stroke cap (butt, round, square)
        lty: u32,    // TODO: stroke type
        lwd: f32,    // stroke width
        lmiter: f32, // stoke miter; Limit at which a sharp corner is drawn beveled.
        blend_mode: u32,
    ) {
        let [r, g, b, a] = col.to_ne_bytes();
        self.paint.set_color(Color::from_argb(a, r, g, b));
        self.paint.set_style(Canvas::fill_to_style(fill));
        self.paint.set_stroke_join(Canvas::ljoin_to_join(ljoin));
        self.paint.set_stroke_cap(Canvas::lend_to_cap(lend));
        self.paint.set_stroke_width(lwd);
        self.paint.set_stroke_miter(lmiter);
        self.paint.set_blend_mode(Canvas::blend_mode(blend_mode));
    }
}
