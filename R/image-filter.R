#' @export
print.ImageFilter <- function(x, ...) {
  cat("ImageFilter::", x$get_label(), "\n", sep = "")
}

#' @export
c.ImageFilter <- function(...) {
  purrr::reduce(list(...), function(acc, nxt) {
    ImageFilter$compose(acc, nxt)
  })
}

#' ImageFilter
#'
#' @description
#' `ImageFilter` is a struct that offers a reference to `skia_safe::ImageFilter`.
#' You can apply an image filter to canvas via [paint()].
#'
#' @details
#' The following filters are available:
#'
#' * `no_filter()`: does not apply any image filter. This is the default image filter for [paint()].
#' * `from_picture(img, crop_rect)`: creates an image filter from a picture.
#' * `compose(outer, inner)`: composes two image filters.
#' * `blend(dst, src, mode, crop_rect)`: blends two image filters with a given blend mode.
#' * `arithmetic(dst, src, coef, crop_rect)`: applies an arithmetic operation to two image filters.
#' * `color_matrix(color_mat)`: creates an image filter from a color matrix.
#' * `displacement_map(channels, scale, displacement, crop_rect)`: creates a displacement map.
#' * `runtime_shader(source, uniforms)`: creates an image filter from a [RuntimeEffect].
#'
#' @param img A raw vector of picture.
#' @param crop_rect Numerics of length 4 for cropping the filtered image.
#' @param outer,inner `ImageFilter` objects.
#' @param dst,src `ImageFilter` objects.
#' @param mode [BlendMode].
#' @param coef Numerics that represents the coefficients `c(k1, k2, k3, k4)`.
#' Each output pixel is the result of combining the corresponding `dst` and `src` pixels using these values.
#' @param color_mat A 5x4 row-major numeric matrix that represents a color matrix.
#' Every pixel's color value is multiplied by this matrix in the same way as the [feColorMatrix](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feColorMatrix) SVG filter.
#' A playground to build color matrices is available [here](https://fecolormatrix.com/).
#' @param channels Numerics of length 2 in range of `[0, 3]` (corresponding to R, G, B, or A channel);
#' color channels to be used along X and Y axes within the source image.
#' @param scale A numeric scalar; displacement scale factor to be used.
#' @param displacement An `ImageFilter` object that displaces the source image.
#' @param source A [RuntimeEffect] object.
#' @param uniforms A named list of numerics to be assigned to uniforms in `source`.
#'
#' @returns An `ImageFilter` object.
#' @seealso
#' * [Image Filters | React Native Skia](https://shopify.github.io/react-native-skia/docs/image-filters/overview/)
#' * [skia_safe::image_filters - Rust](https://rust-skia.github.io/doc/skia_safe/image_filters/index.html)
#' @family paint-attributes
#' @rdname skiagd-image-filter
#' @name ImageFilter
NULL
