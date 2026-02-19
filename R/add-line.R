#' Add lines
#'
#' Adds one or more line segments to an existing picture.
#'
#' @details
#' The number of line segments is determined by `nrow(from)`. The number of rows
#' of `to`, and the lengths of `sigma` and `width` (and the number of columns of
#' `color`) must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all line
#' segments.
#'
#' @param from A numeric matrix (or a data-frame-like object)
#'  with 2 columns (x and y)
#'  where each row represents the start point of a line segment.
#' @param to A numeric matrix (or a data-frame-like object)
#'  with 2 columns (x and y),
#'  where each row represents the end point of a line segment.
#' @inheritParams param-img-and-props
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' lines <-
#'   dplyr::tibble(
#'     x0 = c(50, 50),
#'     y0 = c(50, 250),
#'     x1 = c(350, 350),
#'     y1 = c(50, 250)
#'   )
#'
#' canvas("white") |>
#'   add_line(
#'     from = dplyr::select(lines, x0, y0),
#'     to = dplyr::select(lines, x1, y1)
#'   ) |>
#'   draw_img()
#' }
add_line <- function(img, from, to, ..., props = paint()) {
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], nrow(from))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(from))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], nrow(from)), nrow = 4)
  }
  validate_length(
    nrow(from),
    nrow(to),
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_line(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    from[, 1, drop = TRUE],
    from[, 2, drop = TRUE],
    to[, 1, drop = TRUE],
    to[, 2, drop = TRUE],
    sigma,
    width,
    as.integer(color)
  )
}
