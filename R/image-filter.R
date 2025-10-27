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
#' Concatenating image filters with `c()` is equivalent to sequentially compose them
#' into a single filter using `ImageFilter$compose()`.
#' For blending image filters, use `ImageFilter$blend()` explicitly.
#'
#' @details
#' The following filters are available:
#'
#' * `no_filter()`: does not apply any image filter. This is the default image filter for [paint()].
#' * `from_picture(img, crop_rect)`: creates an image filter from a picture.
#' * `arithmetic(dst, src, coef, crop_rect)`: applies an arithmetic operation to two image filters.
#' * `blend(dst, src, mode, crop_rect)`: blends two image filters with a given blend mode.
#' * `blur(sigma, tile_mode, crop_rect)`: creates a blur image filter.
#' * `color_matrix(color_mat)`: creates an image filter from a color matrix.
#' * `compose(outer, inner)`: composes two image filters.
#' * `crop(crop_rect, tile_mode)`: crops an image filter.
#' * `dilate(radius, crop_rect)`: dilates an image filter.
#' * `displacement_map(channels, scale, displacement, crop_rect)`: creates a displacement map.
#' * `drop_shadow(offset, sigma, color, crop_rect)`: creates a drop shadow image filter.
#' * `erode(raidus, crop_rect)`: erodes an image filter.
#' * `offset(offset, crop_rect)`: creates an offset image filter.
#' * `runtime_shader(source, uniforms)`: creates an image filter from a [RuntimeEffect].
#'
#' @param img A raw vector of picture.
#' @param crop_rect Numerics of length 4 for cropping the filtered image.
#' @param dst,src `ImageFilter` objects.
#' @param coef Numerics that represents the coefficients `c(k1, k2, k3, k4)`.
#' Each output pixel is the result of combining the corresponding `dst` and `src` pixels using these values.
#' @param mode [BlendMode].
#' @param sigma Numerics of length 2 for blur sigma.
#' @param tile_mode [TileMode].
#' @param color_mat A 5x4 row-major numeric matrix that represents a color matrix.
#' Every pixel's color value is multiplied by this matrix in the same way as the [feColorMatrix](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feColorMatrix) SVG filter.
#' A playground to build color matrices is available [here](https://fecolormatrix.com/).
#' @param outer,inner `ImageFilter` objects.
#' @param radius Numerics of length 2; radius of elipse for dilation and erosion.
#' @param channels Numerics of length 2 in range of `[0, 3]` (corresponding to R, G, B, or A channel);
#' color channels to be used along X and Y axes within the source image.
#' @param scale A numeric scalar; displacement scale factor to be used.
#' @param displacement An `ImageFilter` object that displaces the source image.
#' @param offset Numerics of length 2 for X and Y offsets.
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
