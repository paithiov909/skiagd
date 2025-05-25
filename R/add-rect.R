#' Add rectangles
#'
#' @param rect A double matrix where each row is a rectangle
#' XYWH (left, top, right, bottom).
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_rect <- function(img, rect, ..., props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(rect))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(rect))
  }
  validate_length(length(width), rect[, 1])

  sk_draw_rounded_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    rect[, 1],
    rect[, 2],
    rect[, 3],
    rect[, 4],
    rep_len(.0, nrow(rect)),
    rep_len(.0, nrow(rect)),
    width,
    as.integer(color)
  )
}

#' Add rounded rectangles
#'
#' @param rect A double matrix where each row is a rectangle
#' XYWH (left, top, right, bottom).
#' @param radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of oval describing rounded corners.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_rounded_rect <- function(img,
                             rect, radii,
                             ...,
                             props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(rect))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(rect))
  }
  validate_length(
    length(width),
    rect[, 1],
    radii[, 1]
  )
  sk_draw_rounded_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    rect[, 1],
    rect[, 2],
    rect[, 3],
    rect[, 4],
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
  validate_length(
    length(width),
    outer[, 1], outer_radii[, 1],
    inner[, 1], inner_radii[, 1]
  )
  sk_draw_diff_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    outer[, 1],
    outer[, 2],
    outer[, 3],
    outer[, 4],
    outer_radii[, 1],
    outer_radii[, 2],
    inner[, 1],
    inner[, 2],
    inner[, 3],
    inner[, 4],
    inner_radii[, 1],
    inner_radii[, 2],
    width,
    as.integer(color)
  )
}
