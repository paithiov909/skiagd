#' Add PNG image to canvas
#'
#' @inheritParams param-img-and-props
#' @param png A raw vector of PNG data.
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @returns A raw vector of picture.
#' @export
add_png <- function(img, png, left = 0, top = 0, props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_png(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
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

#' Plot picture as PNG image
#'
#' @inheritParams param-img-and-props
#' @returns `img` is returned invisibly.
#' @export
draw_img <- function(img, props = paint()) {
  if (requireNamespace("magick", quietly = TRUE)) {
    img <- as_png(img, props)
    png <- grDevices::as.raster(magick::image_read(img))
    plot(png)
  } else {
    rlang::abort("magick package is required")
  }
  invisible(img)
}
