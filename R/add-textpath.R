#' Add textpath
#'
#' @description
#' Draws text along SVG paths.
#'
#' The return value is often a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always [freeze()] the picture after drawing text.
#'
#' @param path A character vector of SVG notations
#' like `"M10 10 H 90 V 90 H 10 L 10 10"`.
#' @param text Characters to be drawn along `path`.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_textpath <- function(img, path, text, props = paint()) {
  sk_draw_textpath(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    path
  )
}
