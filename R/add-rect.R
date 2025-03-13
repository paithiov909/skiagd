#' Add rectangles
#'
#' @param rect A double matrix where each row is a rectangle corner.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_rect <- function(img, rect, props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_irect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    rect[, 1],
    rect[, 2],
    rect[, 3],
    rect[, 4]
  )
}
