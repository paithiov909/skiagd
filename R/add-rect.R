#' Add rectangles
#'
#' @param xywh A double matrix where each row is a rectangle
#' XYWH (left, top, right, bottom).
#' @param radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of oval describing rounded corners.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_rect <- function(img,
                     xywh, radii = matrix(0, nrow(xywh), 2),
                     ...,
                     props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(xywh))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(xywh))
  }
  validate_length(
    nrow(xywh),
    radii[, 1], radii[, 2], width
  )
  sk_draw_rounded_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    t(xywh[, 1:4]),
    radii[, 1],
    radii[, 2],
    width,
    as.integer(color)
  )
}

#' Add difference rectangles
#'
#' @param outer A double matrix where each row is an outer rectangle
#' XYWH (left, top, right, bottom).
#' @param outer_radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of outer oval describing rounded corners.
#' @param inner A double matrix where each row is an inner rectangle
#' XYWH (left, top, right, bottom).
#' @param inner_radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of inner oval describing rounded corners.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_diff_rect <- function(img,
                          outer, outer_radii,
                          inner, inner_radii,
                          ...,
                          props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(outer))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(outer))
  }
  if (nrow(outer) != nrow(inner)) {
    rlang::abort("outer and inner must have the same number of rows.")
  }
  validate_length(
    nrow(outer),
    outer_radii[, 1], outer_radii[, 2],
    inner_radii[, 1], inner_radii[, 2],
    width
  )
  sk_draw_diff_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    t(outer[, 1:4]),
    outer_radii[, 1],
    outer_radii[, 2],
    t(inner[, 1:4]),
    inner_radii[, 1],
    inner_radii[, 2],
    width,
    as.integer(color)
  )
}
