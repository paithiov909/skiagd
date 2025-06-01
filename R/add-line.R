#' Add lines
#'
#' @param from A double matrix where each row is a start point.
#' @param to A double matrix where each row is an end point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_line <- function(img, from, to, ..., props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(from))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(from))
  }
  validate_length(
    nrow(from),
    nrow(to),
    length(width),
    ncol(color)
  )

  sk_draw_line(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    from[, 1],
    from[, 2],
    to[, 1],
    to[, 2],
    width,
    as.integer(color)
  )
}
