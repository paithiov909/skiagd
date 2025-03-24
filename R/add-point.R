#' Add points
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_point <- function(img, point, props = paint()) {
  sk_draw_points(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    point[, 1],
    point[, 2],
    props[["point_mode"]]
  )
}
