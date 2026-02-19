#' Add circles
#'
#' Adds one or more circles to an existing picture.
#'
#' @details
#' The number of circles is determined by `nrow(center)`. The length of `radius`
#' must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all circles.
#'
#' @param center A numeric matrix (or a data-frame-like object)
#'  with 2 columns (x and y),
#'  where each row represents the center of a circle.
#' @param radius A numeric vector of radii, one for each circle.
#' @inheritParams param-img-and-props
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' circles <-
#'  dplyr::tibble(
#'    x = c(100, 300),
#'    y = c(100, 200),
#'    radius = c(40, 60)
#'  )
#'
#' canvas("white") |>
#'   add_circle(
#'     center = dplyr::select(circles, x, y),
#'     radius = dplyr::pull(circles, radius)
#'   ) |>
#'   draw_img()
#' }
add_circle <- function(img, center, radius, ..., props = paint()) {
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], nrow(center))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(center))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], nrow(center)), nrow = 4)
  }
  validate_length(
    nrow(center),
    length(radius),
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_circle(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    center[, 1, drop = TRUE],
    center[, 2, drop = TRUE],
    radius,
    sigma,
    width,
    as.integer(color)
  )
}
