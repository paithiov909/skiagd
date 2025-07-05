#' Convert picture into native raster
#'
#' @note
#' This function returns integer matrices with the `nativeRaster` class,
#' but they do not strictly conform to R's native raster specification.
#'
#' Since skiagd internally uses premultiplied alpha,
#' these native raster objects may look different
#' than the expected PNG image if the alpha channel is not fully opaque.
#'
#' @inheritParams param-img-and-props
#' @returns A raw vector of PNG image.
#' @export
as_nativeraster <- function(img, ..., props = paint()) {
  sk_as_nativeraster(props[["canvas_size"]], img, props[["transform"]])
}

#' Convert picture into recorded plot
#'
#' This is mainly for testing purposes.
#'
#' @inheritParams param-img-and-props
#' @returns
#' A `recordedplot` object is invisibly returned.
#' See [grDevices::recordPlot()] for details.
#' @export
as_recordedplot <- function(img, ..., props = paint()) {
  rast <- as_nativeraster(img, props = props)
  grid::grid.newpage(recording = FALSE)
  grid::grid.raster(rast, interpolate = TRUE)
  invisible(grDevices::recordPlot())
}

#' Plot picture as a raster
#'
#' @inheritParams param-img-and-props
#' @returns `img` is returned invisibly.
#' @export
draw_img <- function(img, ..., props = paint()) {
  rast <- as_nativeraster(img, props = props)
  grid::grid.newpage(recording = FALSE)
  grid::grid.raster(rast, interpolate = TRUE)
  invisible(img)
}
