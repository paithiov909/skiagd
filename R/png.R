#' Add PNG image to canvas
#'
#' @inheritParams param-img-and-props
#' @param png A raw vector of PNG data.
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @returns A raw vector of picture.
#' @export
add_png <- function(img, png, left = 0, top = 0, props = paint()) {
  sk_draw_png(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    as.integer(c(left, top))
  )
}

#' Convert picture into PNG data
#'
#' @inheritParams param-img-and-props
#' @returns A raw vector of PNG data.
#' @export
as_png <- function(img, props = paint()) {
  sk_as_png(props[["canvas_size"]], img, props[["transform"]])
}

#' Freeze a picture
#'
#' `as_png(img)` and then adds it to a new canvas.
#'
#' @param fill A string scalar; named colors or hexadecimal color codes.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
freeze <- function(img, fill = "transparent", props = paint()) {
  img |>
    as_png(props = props) |>
    add_png(canvas(fill, size = props[["canvas_size"]]), png = _, props = props)
}

#' Plot picture as PNG image
#'
#' @inheritParams param-img-and-props
#' @returns `img` is returned invisibly.
#' @export
draw_img <- function(img, props = paint()) {
  if (!requireNamespace("magick", quietly = TRUE)) {
    rlang::abort("magick package is required")
  }
  png <- as_png(img, props)
  plot(grDevices::as.raster(magick::image_read(png)))
  invisible(img)
}
