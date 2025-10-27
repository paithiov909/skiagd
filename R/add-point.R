#' Add points
#'
#' @note
#' * `sigma`, `width`, and `color` are applied by each `group` for this function.
#' They are expected to have the same length as `group`, not the same length as `point`.
#'
#' @param point A double matrix where each row is a point.
#' @param group Grouping index for `point`. Each group of points is drawn at the same time.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_point <- function(
  img,
  point,
  group = rep_len(1, nrow(point)),
  ...,
  props = paint()
) {
  validate_length(
    nrow(point),
    length(group)
  )
  group <- rle(group)
  len <- length(group[["values"]])

  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], len)
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], len)
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], len), nrow = 4)
  }
  validate_length(
    len,
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_points(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    point[, 1],
    point[, 2],
    group[["lengths"]],
    sigma,
    width,
    as.integer(color),
    props[["point_mode"]]
  )
}
