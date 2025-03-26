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

#' List available font families
#'
#' @description
#' Returns font families available on the system.
#'
#' Since skiagd can only access fonts installed on the system,
#' font families in the font registry or local fonts
#' registered by the [systemfonts](https://systemfonts.r-lib.org/) package
#' cannot be specified as the `family` in [paint()].
#'
#' @returns A tibble.
#' @export
list_font_families <- function() {
  ret <- data.frame(
    family = sk_list_families()
  )
  class(ret) <- c("tbl_df", "tbl", "data.frame")
  ret
}
