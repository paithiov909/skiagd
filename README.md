# skiagd


<!-- README.md is generated from README.Rmd. Please edit that file -->

<!-- badges: start -->

[![rust-skia-version](https://img.shields.io/badge/skia--bindings-v0.88.0-orange)](https://github.com/rust-skia/rust-skia/releases/tag/0.88.0)
[![R-CMD-check](https://github.com/paithiov909/skiagd/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/paithiov909/skiagd/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

skiagd is a toy R wrapper for
[rust-skia](https://github.com/rust-skia/rust-skia) (the Rust crate
[skia_safe](https://rust-skia.github.io/doc/skia_safe/), a binding for
[Skia](https://skia.org/)).

Despite its name, this package is intended as a drawing library, not a
graphics device for R 😓

## Design notes

- This is not a graphics device. skiagd does not allow R’s session to
  hold a reference to a canvas object on Rust side.
- Drawing functions return a
  [picture](https://shopify.github.io/react-native-skia/docs/shapes/pictures/)
  as a `raw` object every time it’s called. `add_*` puts those data onto
  canvas, actually ***adds*** some shapes to there, and then returns a
  `raw` object again.

## Roadmap

I’m planning to re-implement features like [React Native
Skia](https://shopify.github.io/react-native-skia/).

### Implemented

- Shapes
  - Path
    - [x] SVG notation (path)
  - Polygons
    - [x] Rect (round_rect)
    - [x] DiffRect (drrect)
    - [x] Line
    - [x] Points (points; not point)
  - Ellipses
    - [x] Circle
    - [x] Oval / Arc
  - Others
    - [x] Vertices
    - [x] Atlas
- Images
  - [x] PNG
- Text
  - [x] Text Blob
- Painting Attributes
  ([Paint](https://rust-skia.github.io/doc/skia_safe/type.Paint.html))
  - [x] Path Effects
  - [x] Mask Filter (blur)
  - [x] Shaders
  - [ ] Image Filters / Runtime Shaders
    - [x] SkSL and
      [uniforms](https://rust-skia.github.io/doc/skia_safe/runtime_effect/type.RuntimeShaderBuilder.html#method.set_uniform_int)
      support
    - [ ] `*_lit_diffuse` and `*_lit_specular`

### Not planned

- Shapes
  - Patch
- Text
  - Paragraph
  - Text Path
- Fitting Images (needs to re-implement
  [this](https://github.com/Shopify/react-native-skia/blob/4192f839d7ffc5cb0aba91c0f0f97e595d5c8409/packages/skia/cpp/api/recorder/ImageFit.h))
- [Color
  Filters](https://shopify.github.io/react-native-skia/docs/color-filters)
  (only `skia_safe::image_filters::color_filter` is available)
- [Group](https://shopify.github.io/react-native-skia/docs/group/) /
  [Backdrop
  Filters](https://shopify.github.io/react-native-skia/docs/backdrops-filters)
- [Mask](https://shopify.github.io/react-native-skia/docs/mask/)

## Showcase

It’s still in early stage. The API is subject to change.

``` r
pkgload::load_all(export_all = FALSE)
#> ℹ Loading skiagd

set.seed(1234)
size <- dev_size("px")
n_circles <- 250

img_data <-
  canvas("snow") |>
  add_line(
    matrix(c(runif(300, 0, size[1]), runif(300, 0, size[2])), ncol = 2),
    matrix(c(runif(300, 0, size[1]), runif(300, 0, size[2])), ncol = 2),
    props = paint(color = "#fff28166", width = 6)
  ) |>
  add_circle(
    matrix(c(runif(n_circles, 0, size[1]), runif(n_circles, 0, size[2])), ncol = 2),
    runif(n_circles, 6, 50),
    props = paint(color = "#81ffb366", blend_mode = BlendMode$Multiply)
  ) |>
  add_circle(
    matrix(c(runif(n_circles, 0, size[1]), runif(n_circles, 0, size[2])), ncol = 2),
    runif(n_circles, 20, 60),
    props = paint(color = "#f281ff66", blend_mode = BlendMode$Exclusion)
  ) |>
  add_path(
    "M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 155 L 0 93 L 88 80 L 128 0 Z",
    rsx_trans = matrix(c(1, 0, size[1] / 2 - 128, size[2] / 2 - 128, 0, 0), ncol = 6),
    props = paint(color = "gold")
  ) |>
  add_png(
    canvas("transparent") |>
      add_text(
        "Hello, skiagd!",
        rsx_trans = text_info("Hello, skiagd!", props = paint(fontsize = 96, family = "mono")) |>
          dplyr::reframe(
            sc = rep_len(1, n_chars),
            rot = rep_len(0, n_chars),
            x = width / n_chars * (seq_len(n_chars) - 1),
            y = rep_len(96, n_chars),
            ax = rep_len(0, n_chars),
            ay = rep_len(0, n_chars),
            .by = id
          ) |>
          dplyr::select(!"id") |>
          as.matrix(),
        props = paint(fontsize = 96, family = "mono", fontface = FontStyle$Bold, color = "maroon")
      ) |>
      as_png(),
    left = size[1] / 2 - text_info("Hello, skiagd!", props = paint(fontsize = 96))[["width"]] / 2,
    top = size[2] / 9 * 6
  ) |>
  as_png()

## `as_png()` returns a PNG image with alpha channel as a raw vector.
## You can save it to a PNG file using `writeBin()`.
# writeBin(img_data, "man/figures/README-test-plot.png")

## Here we convert it to JPEG to save file size.
magick::image_read(img_data) |>
  magick::image_scale("720") |>
  magick::image_convert("jpeg") |>
  magick::image_write("man/figures/README-test-plot-1.jpg")
```

![README-test-plot-1](man/figures/README-test-plot-1.jpg)

``` r
img_data <-
  canvas("darkslateblue") |>
  add_rect(
    matrix(c(0, 0, size[1], size[2]), ncol = 4),
    props = paint(
      blend_mode = BlendMode$Lighten,
      sytle = Style$Fill,
      shader = Shader$conical_gradient(
        c(size[1] / 2 * .8, size[2] / 2 * .8),
        c(size[1] / 2 * .2, size[2] / 2 * .2),
        c(size[1] / 2 * .8, size[1] / 2 * .2),
        color = grDevices::col2rgb(c("blueviolet", "skyblue"), alpha = TRUE),
        mode = TileMode$Clamp,
        flags = FALSE,
        transform = c(1, 0, 0, 0, 1, 0, 0, 0, 1)
      )
    )
  ) |>
  add_circle(
    matrix(c(size[1] / 2, size[2]), ncol = 2), size[1] * .4,
    props = paint(
      blend_mode = BlendMode$HardLight,
      style = Style$Stroke,
      cap = Cap$Square,
      path_effect = PathEffect$line_2d(12, c(12, 0, 0, 0, 32, 0, 0, 0, 1)),
      shader = Shader$sweep_gradient(
        c(size[1] / 2, size[2]),
        0, 360,
        color = grDevices::col2rgb(c("magenta", "gold"), alpha = TRUE),
        mode = TileMode$Clamp,
        flags = FALSE,
        transform = c(1, 0, 0, 0, 1, 0, 0, 0, 1)
      )
    )
  ) |>
  as_png()

magick::image_read(img_data) |>
  magick::image_scale("720") |>
  magick::image_convert("jpeg") |>
  magick::image_write("man/figures/README-test-plot-2.jpg")
```

![README-test-plot-2](man/figures/README-test-plot-2.jpg)

The following is a short animation of ‘Mystery rose’. The code is based
on [this
post](https://georgemsavva.github.io/creativecoding/posts/mystery/).

``` r
library(gifski)

circle <- function(amp, freq, phase) {
  amp * 1i^(freq * seq(0, 600, length.out = 260) + phase)
}
li <- seq(0, 1, length.out = 30)[-1]

size <- c(120, 120) / 2
trans <- matrix(c(20, 0, size[1], 0, 20, size[2], 0, 0, 1), ncol = 3)

save_gif(lapply(seq_along(li), function(ai) {
  a <- li[ai] * 5
  l <- sin(pi * (2 * a - .5)) + 1

  z <- circle(1, 1, 0) +
    circle(l, ceiling(a), -8 * a) +
    circle(l / 2 - 1, ceiling(((-a + 2.5) %% 5) - 5), -4 * a)
  z2 <- c(z[-1], z[1])

  hue <- (a + (Re(z / 10))) %% 1
  colors <- grDevices::hsv(hue, 0.65, 1, alpha = 1)

  canvas("#04010F") |>
    add_circle(
      matrix(c(Re(z), Im(z), rep_len(1, length(z))), ncol = 3) %*% trans,
      rep_len(2, length(z)),
      color = grDevices::col2rgb(colors, alpha = TRUE)
    ) |>
    add_line(
      matrix(c(Re(z), Im(z), rep_len(1, length(z))), ncol = 3) %*% trans,
      matrix(c(Re(z2), Im(z2), rep_len(1, length(z))), ncol = 3) %*% trans,
      width = rep(0.1 + seq(0, 1, length.out = length(z) / 10), each = 10),
      color = grDevices::col2rgb(colors, alpha = TRUE)
    ) |>
    draw_img()

}), delay = 1 / 6, width = 120, height = 120, progress = TRUE)
```

<img src="man/figures/animation.gif" alt="mystery-rose" width="50%" />
