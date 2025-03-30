#' Add textpath
#'
#' @description
#' Draws text along SVG paths.
#'
#' The return value is often a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always [freeze()] the picture after drawing text.
#'
#' @details
#' Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#'
#' @param text Characters to be drawn along `path`.
#' @param path A character vector of SVG notations
#' like `"M10 10 H 90 V 90 H 10 L 10 10"`.
#' @param transform Numerics of length 9
#' to apply affine transformations to the path themselves.
#' See [transform-matrix] for affine transformations.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_textpath <- function(img, text, path,
                         transform = diag(1, 3),
                         props = paint()) {
  sk_draw_textpath(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    path,
    transform
  )
}
