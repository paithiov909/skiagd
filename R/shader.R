#' @export
print.Shader <- function(x, ...) {
  cat("Shader::", x$get_label(), "\n", sep = "")
}

#' @export
c.Shader <- function(..., mode = paint()[["blend_mode"]]) {
  purrr::reduce(list(...), function(acc, nxt) {
    Shader$blend(mode, nxt, acc)
  })
}

#' Shader
#'
#' @description
#' `Shader` is a struct that offers a reference to `skia_safe::Shader`.
#' You can apply a shader to shapes via [paint()].
#'
#' Concatenating shaders with `c()` is equivalent to blend them all
#' into a single shader using `Shader$blend()` with the default `BlendMode`.
#' You can pass `mode` explicitly for `c()` to change the blend mode.
#'
#' @details
#' The following shaders are available:
#'
#' * `no_shader()`: does not apply any shader. This is the default shader for [paint()].
#' * `from_picture(img, mode, tile_size, transform)`: takes a picture and returns an image shader.
#' * `from_png(png, mode, transform)`: takes a PNG image and returns an image shader.
#' * `from_runtime_effect(source, uniforms)`: takes a [RuntimeEffect] and returns a shader.
#' * `color(color)`: takes a color and returns a color shader.
#' * `blend(mode, dst, src)`: returns a shader where the given shaders are combined with [BlendMode].
#' * `fractal_noise(freq, octaves, seed, tile_size)`: fractal perlin noise shader.
#' * `turbulence(freq, octaves, seed, tile_size)`: turbulence noise shader.
#' * `linear_gradient(start, end, color, mode, flags, transform)`: linear gradient shader.
#' * `radial_gradient(center, radius, color, mode, flags, transform)`: radial gradient shader.
#' * `conical_gradient(start, end, radii, color, mode, flags, transform)`: conical gradient shader.
#' * `sweep_gradient(center, start_angle, end_angle, color, mode, flags, transform)`: sweep gradient shader.
#'
#' @param img A raw vector of picture.
#' @param mode For `blend()`, [BlendMode]. For others, [TileMode].
#' @param tile_size Numerics of length 2; tile size (width, height).
#' @param transform Numerics of length 9; transformation matrix.
#' @param png A raw vector of PNG image.
#' @param source A [RuntimeEffect] object.
#' @param uniforms A named list of numerics to be assigned to uniforms in `source`.
#' @param color An integer matrix in range `[0, 255]` where each column is an RGBA color.
#' @param dst A `Shader` object; destination shader.
#' @param src A `Shader` object; source shader.
#' @param freq Numerics of length 2; frequencies.
#' @param octaves A numeric scalar; number of octaves.
#' @param seed Integer scalar; random seed.
#' @param start Numerics of length 2; starting point (x, y).
#' @param end Numerics of length 2; ending point (x, y).
#' @param flags A logical scalar; typically, you can leave this as `FALSE`.
#'  See [here](https://shopify.github.io/react-native-skia/docs/shaders/gradients/#common-properties)
#'  for details.
#' @param radii Numerics of length 2; radii of start and end circles.
#' @param center Numerics of length 2; center of the gradient.
#' @param start_angle A numeric scalar in range `[0, 360]`;
#'  starting angle. For default, set `0`.
#' @param end_angle A numeric scalar in range `[0, 360]`;
#'  ending angle. For default, set `360`.
#'
#' @returns A `Shader` object.
#' @seealso
#' * [Image Shaders | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/images)
#' * [Gradients | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/gradients)
#' * [Perlin Noise Shaders | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/perlin-noise)
#' * [Blending and Colors | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/colors)
#' @family paint-attributes
#' @rdname skiagd-shader
#' @name Shader
NULL
