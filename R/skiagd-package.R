## usethis namespace: start
#' @useDynLib skiagd, .registration = TRUE
## usethis namespace: end
#' @keywords internal
"_PACKAGE"

#' Color to RGBA
#'
#' A wrapper of [grDevices::col2rgb()].
#' In general, you don't need to use this function explicitly.
#'
#' @param color `col` for [grDevices::col2rgb()].
#' @returns an integer vector.
#' @export
col2rgba <- function(color) {
  as.vector(grDevices::col2rgb(color, alpha = TRUE))[1:4]
}

#' Device size
#'
#' Just returns the size of the current device as an integer (not a numeric).
#'
#' @param units `units` for [grDevices::dev.size()].
#' @returns an integer vector.
#' @export
dev_size <- function(units = "px") {
  as.integer(grDevices::dev.size(units))
}

#' @export
with_group <- function(expr, ...) {
  # TODO
}

#' Paint props
#'
#' @param ... <[`dynamic-dots`][rlang::dyn-dots]> What these dots do.
#' @returns a list.
#' @export
paint <- function(...) {
  default_props <- unclass(grid::get.gpar())
  default_props[["blend_mode"]] <- 1 # NOTE: 0~27

  dots <- rlang::list2(...)
  props <-
    purrr::list_assign(
      default_props,
      !!!dots
    )
  list(
    col = col2rgba(props[["col"]]),
    fill = col2rgba(props[["fill"]]),
    ljoin = switch(props[["linejoin"]],
      "round" = 1,
      "mitre" = 2,
      "bevel" = 3
    ),
    lend = switch(props[["lineend"]],
      "round" = 1,
      "butt" = 2,
      "square" = 3
    ),
    lty = 0L, # FIXME: not used.
    lwd = props[["lwd"]],
    lmiter = props[["linemitre"]],
    blend_mode = props[["blend_mode"]]
  )
}

#' Create a canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill a color name.
#' @returns a raw vector.
#' @export
canvas <- function(fill = paint()[["fill"]], .size = dev_size()) {
  sk_absolute_fill(.size, col2rgba(fill))
}

#' Add a circle
#'
#' @param img a raw vector.
#' @param x a numeric.
#' @param y a numeric.
#' @param r a numeric.
#' @param props a paint prop.
#' @param .size an integer vector.
#' @returns a raw vector.
#' @export
add_circle <- function(img, x, y, r, props = paint(), .size = dev_size()) {
  # TODO: validate img?
  sk_circle(.size, img, x, y, r, props)
}

#' Save image to file
#'
#' @param img a raw vector.
#' @param filename a character string.
#' @param .size an integer vector.
#' @returns `filename` is invisibly returned.
#' @export
save_img <- function(img, filename, .size = dev_size()) {
  invisible(sk_save_png(.size, img, filename))
}

#' @export
draw_img <- function(img) {
  if (requireNamespace("magick", quietly = TRUE)) {
    png <- grDevices::as.raster(magick::image_read(img))
    plot(png)
  } else {
    stop("magick package is required")
  }
}
