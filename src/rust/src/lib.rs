use std::fs::File;
use std::io::Write;

use cxx::CxxVector;

use skia_safe::{surfaces, Color, Data, EncodedImageFormat, Image, Paint, Surface};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Canvas;
        /// Calls `surface.canvas().clear(Color::TRANSPARENT);`
        fn skia_canvas(width: i32, height: i32) -> Box<Canvas>;
        /// Reads bytes and draws them to the canvas. Paint is not applied here.
        unsafe fn read_bytes(&mut self, bytes: &CxxVector<u8>, left: i32, top: i32) -> Result<()>;
        /// Returns bytes from the canvas
        fn save_bytes(&mut self) -> Vec<u8>;
        /// Save the canvas as png
        fn save_png(&mut self, filename: String) -> Result<()>;
        /// Fills the canvas with `col`
        fn new_page(&mut self, col: u32);
        fn points(&mut self, x: f32, y: f32, mode: u32);
        fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32);
        fn circle(&mut self, x: f32, y: f32, r: f32);
        fn irect(&mut self, left: i32, top: i32, right: i32, bottom: i32);
        fn svg_path(&mut self, svg: &str) -> Result<()>;
        fn translate(&mut self, dx: f32, dy: f32);
        fn scale(&mut self, sx: f32, sy: f32);
        /// Applys properties to the paint
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
    surface: Surface,
    paint: Paint,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = surfaces::raster_n32_premul((width, height))
            .unwrap_or_else(|| surfaces::raster_n32_premul((720, 576)).unwrap());
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        surface.canvas().clear(Color::TRANSPARENT);
        Canvas { surface, paint }
    }

    #[inline]
    pub unsafe fn read_bytes(
        &mut self,
        bytes: &CxxVector<u8>,
        left: i32,
        top: i32,
    ) -> anyhow::Result<()> {
        let input = Data::new_bytes(bytes.as_slice());
        let image = Image::from_encoded_with_alpha_type(input, skia_safe::AlphaType::Premul)
            .ok_or_else(|| return anyhow::Error::msg("Bomb! failed to read bytes"))?;
        self.surface.canvas().draw_image(&image, (left, top), None);
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

    pub fn points(&mut self, x: f32, y: f32, mode: u32) {
        let mode = match mode {
            0 => skia_safe::canvas::PointMode::Points,
            1 => skia_safe::canvas::PointMode::Lines,
            2 => skia_safe::canvas::PointMode::Polygon,
            _ => skia_safe::canvas::PointMode::Points,
        };
        self.surface
            .canvas()
            .draw_points(mode, &[skia_safe::Point::new(x, y)], &self.paint);
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.surface
            .canvas()
            .draw_line((x1, y1), (x2, y2), &self.paint);
    }

    pub fn circle(&mut self, x: f32, y: f32, r: f32) {
        self.surface.canvas().draw_circle((x, y), r, &self.paint);
    }

    pub fn irect(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        let rect = skia_safe::IRect::new(left, top, right, bottom);
        self.surface.canvas().draw_irect(rect, &self.paint);
    }

    pub fn svg_path(&mut self, svg: &str) -> anyhow::Result<()> {
        let path = skia_safe::utils::parse_path::from_svg(svg)
            .ok_or_else(|| return anyhow::Error::msg("Bomb! failed to parse svg"))?;
        self.surface.canvas().draw_path(&path, &self.paint);
        Ok(())
    }

    pub fn test(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
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

    #[inline]
    fn fill_to_style(fill: u32) -> skia_safe::PaintStyle {
        match fill {
            1 => skia_safe::PaintStyle::StrokeAndFill,
            2 => skia_safe::PaintStyle::Stroke,
            3 => skia_safe::PaintStyle::Fill,
            _ => skia_safe::PaintStyle::StrokeAndFill,
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
