#' Convert picture into native raster
#'
#' @description
#' Converts `img` to an integer matrix with class `nativeRaster`.
#'
#' @details
#' In base R, a `nativeRaster` object conventionally stores
#' colors with ***non-premultiplied*** alpha
#' (RGB channels are independent of the alpha channel).
#'
#' In contrast, skiagd renders internally with ***premultiplied*** alpha,
#' and the resulting pixel values are returned as-is.
#'
#' When such a raster is drawn onto an R graphics device
#' (e.g., via [grid::grid.raster()]),
#' the device composites the image with its background color.
#' If the alpha channel is not fully opaque (< 255),
#' this extra alpha blending can change the apparent colors compared
#' with what you would expect from the output of [as_png()].
#'
#' @inheritParams param-img-and-props
#' @returns A `nativeRaster` object.
#' @export
#' @examples
#' \dontrun{
#' img <- canvas("navy") |>
#'  as_nativeraster()
#'
#' grid::grid.newpage()
#' grid::grid.raster(img, interpolate = FALSE)
#' }
as_nativeraster <- function(img, ..., props = paint()) {
  sk_as_nativeraster(props[["canvas_size"]], img, props[["transform"]])
}

#' Convert picture into recorded plot
#'
#' This function exists for testing purposes.
#' You typically does not need to call this function.
#'
#' @inheritParams param-img-and-props
#' @returns
#' A `recordedplot` object is invisibly returned.
#' See [grDevices::recordPlot()] for details.
#' @export
#' @keywords internal
as_recordedplot <- function(img, ..., props = paint()) {
  rast <- as_nativeraster(img, props = props)
  grid::grid.newpage(recording = FALSE)
  grid::grid.raster(rast, interpolate = TRUE)
  invisible(grDevices::recordPlot())
}

#' Plot picture as raster image
#'
#' @description
#' Converts `img` to a `nativeRaster` and draws it using
#' [grid::grid.raster()] with `interpolate = TRUE`.
#'
#' Linear interpolation is therefore applied by default. This produces
#' smoother edges (e.g., for circles and diagonal shapes), at the cost of
#' slightly slower rendering. If you prefer a faster but more blocky result,
#' you can manually call [grid::grid.raster()] with `interpolate = FALSE`
#' on the output of [as_nativeraster()].
#'
#' See the examples in [as_nativeraster()] for a comparison.
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
