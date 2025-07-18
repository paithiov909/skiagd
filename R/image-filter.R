#' @export
print.ImageFilter <- function(x, ...) {
  cat("ImageFilter::", x$get_label(), "\n", sep = "")
}

# c.ImageFilter <- function(...) {
#   purrr::reduce(list(...), function(acc, nxt) {
#     ImageFilter$compose(acc, nxt)
#   })
# }

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
#' * `runtime_shader(source, uniforms)`: Creates an image filter from a [RuntimeEffect].
#'
#' @param source A [RuntimeEffect] object.
#' @param uniforms A named list of numerics to be assigned to uniforms in `source`.
#'
#' @returns An `ImageFilter` object.
#' @seealso
#' [Image Filters | React Native Skia](https://shopify.github.io/react-native-skia/docs/image-filters/overview/)
#' @family paint-attributes
#' @rdname skiagd-image-filter
#' @name ImageFilter
NULL
