#' Add circles
#'
#' @param center A double matrix where each row is circle center.
#' @param radius Numerics of circle radius.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
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
    center[, 1],
    center[, 2],
    radius,
    sigma,
    width,
    as.integer(color)
  )
}
