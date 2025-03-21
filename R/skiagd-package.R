## usethis namespace: start
#' @importFrom rlang env_get %||%
## usethis namespace: end
#' @keywords internal
"_PACKAGE"

#' Convert a picture into a recorded plot
#'
#' This is mainly for testing purposes.
#'
#' @inheritParams param-img-and-props
#' @returns A `recordedplot` object. See [grDevices::recordPlot()] for details.
#' @export
as_recordedplot <- function(img, props = paint()) {
  if (!requireNamespace("magick", quietly = TRUE)) {
    rlang::abort("magick package is required")
  }
  props <- getOption(".skiagd_paint_group") %||% props
  png <- as_png(img, props)
  graphics::plot.new()
  grid::grid.raster(magick::image_read(png))
  grDevices::recordPlot(load = "skiagd")
}

#' Paint group
#'
#' Evaluates `code` with `props`.
#'
#' @param props Paint properties.
#' @param code Code to evaluate.
#' @export
with_group <- function(props, code) {
  # TODO: check if this can be nested
  withr::with_options(
    list(.skiagd_paint_group = props),
    code
  )
}
