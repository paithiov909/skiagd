#' RuntimeEffect
#'
#' @description
#' `RuntimeEffect` is a struct that wraps `skia_safe::RuntimeEffect`.
#'
#' Skia provides a shading language called SkSL.
#' The syntax is similar to GLSL, but differs in minor details.
#' A quick overview of their differences
#' can be found [here](https://github.com/google/skia/tree/main/src/sksl#readme).
#'
#' You can compile a SkSL source into a `RuntimeEffect`
#' using `RuntimeEffect$make()`,
#' and apply it to the canvas as an [ImageFilter].
#'
#' @details
#' `RuntimeEffect` as an R environment exposes the following method:
#'
#' * `make(sksl)`: Takes a SkSL source and compiles it into a `RuntimeEffect`.
#'
#' A `RuntimeEffect` object has the following method:
#'
#' * `source()`: Returns the original SkSL source as a string scalar.
#'
#' @param sksl A string scalar of SkSL source.
#' The fragment shader must receive the currently filtered image
#' as `shader` uniform.
#' @returns For `make()`, a `RuntimeEffect` object is returned
#' if the SkSL source is successfully compiled.
#' Otherwise, an error is thrown with the compilation error message.
#'
#' @examples
#' \donttest{
#' effect <-
#'   RuntimeEffect$make(R"{
#'     uniform shader image;
#'     uniform vec2 resolution;
#'     vec4 main(vec2 fragCoord) {
#'       vec2 uv = fragCoord / resolution;
#'       return vec4(uv.x, uv.y, .6, 1.0);
#'     }
#'  }")
#'
#' canvas_size <- dev_size()
#' imgf <-
#'   ImageFilter$runtime_shader(
#'     effect,
#'     list(resolution = as.double(canvas_size))
#'   )
#' canvas() |>
#'   add_rect(
#'     matrix(c(0, 0, canvas_size), ncol = 4),
#'     props = paint(image_filter = imgf)
#'   ) |>
#'   draw_img()
#' }
#' @rdname skiagd-runtime-effect
#' @name RuntimeEffect
NULL
