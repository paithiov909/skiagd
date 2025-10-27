#' Add rectangles
#'
#' @param ltrb A double matrix where each row is a rectangle
#' LTRB (left, top, right, bottom).
#' @param radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of oval describing rounded corners.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_rect <- function(
  img,
  ltrb,
  rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), nrow(ltrb), 6, byrow = TRUE),
  radii = matrix(0, nrow(ltrb), 2),
  ...,
  props = paint()
) {
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], nrow(ltrb))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(ltrb))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], nrow(ltrb)), nrow = 4)
  }
  validate_length(
    nrow(ltrb),
    nrow(rsx_trans),
    nrow(radii),
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_rounded_rect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    t(ltrb[, 1:4]),
    radii[, 1],
    radii[, 2],
    t(rsx_trans[, 1:6]),
    sigma,
    width,
    as.integer(color)
  )
}

#' Add difference rectangles
#'
#' @param outer,inner A double matrix where each row is a rectangle
#' LTRB (left, top, right, bottom).
#' @param outer_radii,inner_radii A double matrix where each row is a pair of axis lengths
#' on X-axis and Y-axis of oval describing rounded corners.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_diff_rect <- function(
  img,
  outer,
  inner,
  rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), nrow(outer), 6, byrow = TRUE),
  outer_radii = matrix(0, nrow(outer), 2),
  inner_radii = matrix(0, nrow(inner), 2),
  ...,
  props = paint()
) {
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], nrow(outer))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(outer))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], nrow(outer)), nrow = 4)
  }
  validate_length(
    nrow(outer),
    nrow(inner),
    nrow(rsx_trans),
    nrow(outer_radii),
    nrow(inner_radii),
    length(sigma),
    length(width),
    ncol(color)
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
    t(rsx_trans[, 1:6]),
    sigma,
    width,
    as.integer(color)
  )
}
