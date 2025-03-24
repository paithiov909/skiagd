#' Add circles
#'
#' @param center A double matrix where each row is circle center.
#' @param radius Numerics of circle radius.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_circle <- function(img, center, radius, props = paint()) {
  sk_draw_circle(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    center[, 1],
    center[, 2],
    radius
  )
}
