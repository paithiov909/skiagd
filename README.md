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
- [Painting \| React Native
  Skia](https://shopify.github.io/react-native-skia/docs/paint/overview)

**Design notes**:

- This is not a graphics device. skiagd does not allow R’s session to
  hold a reference to a Canvas object on Rust side.
- Drawing functions return a PNG image as a `raw` object every time it’s
  called. `add_*` puts those data onto canvas, actually ***adds*** some
  paintings to there, and then returns a `raw` object again.
- Uses [cxx](https://github.com/dtolnay/cxx) and cpp11 package. Not
  extendr or savvy.
  - I’m not sure if this is a good idea or not, because extendr supports
    conversion between `raw` and `&[u8]`.

## Use Case?

It’s still in veeeeeery early stage. The API is subject to (possibly
drastic) change.

``` r
pkgload::load_all(export_all = FALSE)
#> ℹ Loading skiagd

img_data <-
  unigd::ugd_render_inline({
    size <- dev_size("px")
    pos_x <- size[1] / 2
    pos_y <- size[2] / 2
    canvas("white") |>
      add_circle(pos_x, pos_y, 120, props = paint(col = "skyblue")) |>
      # 'deeppink' with alpha channel, blend mode 'Overlay'.
      add_circle(pos_x, pos_y, 200, props = paint(col = "#ff1493aa", blend_mode = 15)) |>
      draw_img()
  }, as = "png", width = 720, height = 576)

img_data |>
  magick::image_read() |>
  as.raster() |>
  plot()
```

<img src="man/figures/README-test-plot-1.png" style="width:100.0%" />
