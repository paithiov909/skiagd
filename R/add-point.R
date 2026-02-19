#' Add points
#'
#' Draws grouped point sequences using Skia's batched point API.
#'
#' @details
#' This function draws one shape per group, where each group is a consecutive run
#' in `group` (because grouping is implemented via [rle()]). If the same group id
#' appears in multiple non-consecutive runs, they are treated as separate groups.
#'
#' The drawing behavior for each group is controlled by `props[["point_mode"]]`:
#' * `PointMode$Points`: draws the group's points.
#' * `PointMode$Lines`: draws line segments through the group's points.
#' * `PointMode$Polygon`: draws a polygon from the group's points.
#'
#' The attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' They are applied per group, so their lengths must match the number of groups
#' (i.e., `length(rle(group)$values)`). If not supplied, they are taken from `props`
#' and recycled to all groups.
#'
#' @param point A numeric matrix (or a data-frame-like object) with two numeric
#'  columns (x and y), where each row is a point.
#' @param group A vector of grouping indices for `point`. Points are split into
#'  groups and drawn group-by-group. Note that grouping is based on consecutive
#'  runs of the same value (see Details).
#' @inheritParams param-img-and-props
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' rad <- \(deg) deg * (pi / 180)
#'
#' cv_size <- dev_size()
#'
#' # Generate coordinates for a rose curve
#' rose <-
#'   dplyr::tibble(
#'     i = seq_len(360),
#'     r = 120 * abs(sin(rad(4 * i)))
#'   ) |>
#'   dplyr::reframe(
#'     id = i,
#'     x = r * cos(rad(360 * i / 360)) + cv_size[1] / 2,
#'     y = r * sin(rad(360 * i / 360)) + cv_size[2] / 2,
#'     .by = id
#'   )
#'
#' # Points (color per group; here, one point per group => color per point)
#' canvas("white") |>
#'   add_point(
#'     dplyr::select(rose, x, y),
#'     group = dplyr::pull(rose, id),
#'     color = seq(0, 1, length.out = nrow(rose)) |>
#'       grDevices::hsv(1, 1, 1) |>
#'       col2rgba(),
#'     props = paint(width = 3)
#'   ) |>
#'   draw_img()
#'
#' # Lines (one polyline per group)
#' rose2 <-
#'   dplyr::reframe(
#'     rose,
#'     x = c(cv_size[1] / 2, x),
#'     y = c(cv_size[2] / 2, y),
#'     .by = id
#'   )
#' canvas("white") |>
#'   add_point(
#'     dplyr::select(rose2, x, y),
#'     group = dplyr::pull(rose2, id),
#'     color = unique(dplyr::pull(rose2, id) / nrow(rose)) |>
#'       grDevices::hsv(1, 1, 1) |>
#'       col2rgba(),
#'     props = paint(point_mode = PointMode$Lines, width = 1)
#'   ) |>
#'   draw_img()
#'
#' # Polygon (a single polygon from all points)
#' canvas("white") |>
#'   add_point(
#'     dplyr::select(rose, x, y),
#'     group = rep_len(1, nrow(rose)),
#'     props = paint(point_mode = PointMode$Polygon, width = 3, color = "hotpink")
#'   ) |>
#'   draw_img()
#' }
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
    point[, 1, drop = TRUE],
    point[, 2, drop = TRUE],
    group[["lengths"]],
    sigma,
    width,
    as.integer(color),
    props[["point_mode"]]
  )
}
