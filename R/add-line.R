#' Add lines
#'
#' @param from A double matrix where each row is a start point.
#' @param to A double matrix where each row is an end point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_line <- function(img, from, to, props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_line(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    from[, 1],
    from[, 2],
    to[, 1],
    to[, 2]
  )
}
