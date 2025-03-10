# skiagd


<!-- README.md is generated from README.Rmd. Please edit that file -->

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
<!-- badges: end -->

skiagd is a toy R wrapper for
[rust-skia](https://github.com/rust-skia/rust-skia) (the rust crate
[skia_safe](https://rust-skia.github.io/doc/skia_safe/), a binding for
[Skia](https://skia.org/)).

Despite its name, this package is intended as a graphics library, not a
graphics device for R 😓

## Resources

- [skia_safe - Rust](https://rust-skia.github.io/doc/skia_safe/)
- [Rust ❤️ C++](https://cxx.rs/)
- [cxx_build - Rust](https://docs.rs/cxx-build/latest/cxx_build/)
- [Get started with cpp11 •
  cpp11](https://cpp11.r-lib.org/articles/cpp11.html)

### Design notes

- This is not a graphics device. skiagd does not allow R’s session to
  hold a reference to a Canvas object on Rust side.
- Drawing functions return a PNG image as a `raw` object every time it’s
  called. `add_*` puts those data onto canvas, actually ***adds*** some
  paintings to there, and then returns a `raw` object again.
- Uses [cxx](https://github.com/dtolnay/cxx) and cpp11 package. Not
  extendr or savvy.
  - I’m not sure if this is a good idea or not, because both extendr and
    savvy support `raw`.

### Plans??

I’m planning to re-implement features such like [React Native
Skia](https://shopify.github.io/react-native-skia/).

- Shapes
  - Path
    - [x] SVG notation (path)
    - [ ] trim
    - [ ] fillType
  - Polygons
    - [x] Rect (irect)
    - [ ] RoundedRect (round_rect)
    - [ ] DiffRect (drrect)
    - [x] Line
    - [ ] Points (points; not point)
  - Ellipses
    - [x] Circle
    - [ ] Oval (oval)
    - [ ] Arc (arc)
  - Atlas
  - Vertices
  - Patch
  - Picture
    - [ ] `draw_picture`, `from_data`, and `serialize`
- Images
- Text
  - Paragraph
  - Text
  - Glyphs
  - Text Path
  - Text Blob
- Mask
- Other Paint props
  - [ ] PathEffect
  - [ ] Filters
  - [ ] Shaders??

## Use Case?

It’s still in veeeeeery early stage. The API is subject to (possibly
drastic) change.

``` r
pkgload::load_all(export_all = FALSE)
#> ℹ Loading skiagd

img_data <-
  unigd::ugd_render_inline({
    set.seed(1234)
    size <- dev_size("px")
    n_circles <- 250
    canvas("snow") |>
      add_line(
        matrix(c(runif(300, 0, size[1]), runif(300, 0, size[2])), ncol = 2),
        matrix(c(runif(300, 0, size[1]), runif(300, 0, size[2])), ncol = 2),
        props = paint(col = "#fff28166", lwd = 6)
      ) |>
      # 'skyblue' with alpha channel, blend mode 'Exclusion'.
      add_circle(
        matrix(c(runif(n_circles,  0, size[1]), runif(n_circles, 0, size[2])), ncol = 2),
        runif(n_circles, 6, 50),
        props = paint(col = "#87ceeb66", blend_mode = 23)
      ) |>
      # 'deeppink' with alpha channel, blend mode 'Overlay'.
      add_circle(
        matrix(c(runif(n_circles, 0, size[1]), runif(n_circles, 0, size[2])), ncol = 2),
        runif(n_circles, 20, 60),
        props = paint(col = "#ff1493aa", blend_mode = 15)
      ) |>
      add_path(
        "M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 155 L 0 93 L 88 80 L 128 0 Z",
        translate = size / 2 - c(128L, 128L),
        props = paint(col = "#fff281ee")
      ) |>
      draw_img()
  }, as = "png", width = 1280, height = 720)

img_data |>
  magick::image_read() |>
  as.raster() |>
  plot()
```

<img src="man/figures/README-test-plot-1.png" style="width:100.0%" />
