#' Add PNG image to canvas
#'
#' @inheritParams param-img-and-props
#' @param png A raw vector of PNG image.
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @returns A raw vector of picture.
#' @export
add_png <- function(img, png, left = 0, top = 0, ..., props = paint()) {
  sk_draw_png(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    as.integer(c(left, top))
  )
}

#' Convert picture into PNG image
#'
#' @inheritParams param-img-and-props
#' @returns A raw vector of PNG image.
#' @export
as_png <- function(img, ..., props = paint()) {
  sk_as_png(props[["canvas_size"]], img, props[["transform"]])
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
  if (!requireNamespace("fastpng", quietly = TRUE)) {
    rlang::abort("fastpng package is required")
  }
  png <- as_png(img, props = props) |>
    fastpng::read_png(type = "nativeraster", rgba = TRUE)
  grid::grid.newpage(recording = FALSE)
  grid::grid.raster(png)
  invisible(grDevices::recordPlot())
}

#' Plot picture as a raster
#'
#' @inheritParams param-img-and-props
#' @returns `img` is returned invisibly.
#' @export
draw_img <- function(img, ..., props = paint()) {
  if (!requireNamespace("fastpng", quietly = TRUE)) {
    rlang::abort("fastpng package is required")
  }
  png <- as_png(img, props = props) |>
    fastpng::read_png(type = "nativeraster", rgba = TRUE)
  grid::grid.newpage(recording = FALSE)
  grid::grid.raster(png)
  invisible(img)
}

#' Freeze picture
#'
#' `as_png(img, props)` and then adds it to a new canvas
#' with the default blend mode (`BlendMode$SrcOver`).
#'
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @param fill RGBA representation of a color.
#' This can be specified using named colors or hexadecimal color codes,
#' which are converted internally using [grDevices::col2rgb()].
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
freeze <- function(img, left = 0, top = 0, fill = "transparent", ..., props = paint()) {
  img |>
    as_png(props = props) |>
    add_png(
      canvas(fill = fill, canvas_size = props[["canvas_size"]]),
      png = _,
      left = left,
      top = top
    )
}
