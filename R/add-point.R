#' Add points
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_point <- function(img, point, props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_points(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    point[, 1],
    point[, 2],
    props[["point_mode"]]
  )
}
