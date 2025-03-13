#' Add circles
#'
#' @param center A double matrix where each row is circle center.
#' @param radius A double vector of circle radius.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_circle <- function(img, center, radius, props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_circle(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    center[, 1],
    center[, 2],
    radius
  )
}
