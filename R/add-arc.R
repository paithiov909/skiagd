#' Add arcs
#'
#' @param ltrb A double matrix where each row is a rectangle
#' LTRB (left, top, right, bottom).
#' Each rectangle bounds the area of oval containing arc to draw.
#' @param angle A double matrix where each row is a pair of sweeping angles (in degrees).
#' @param use_center Whether to draw a wedge that includes lines from oval center to arc end points.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_arc <- function(img, ltrb,
                    rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), nrow(ltrb), 6, byrow = TRUE),
                    angle = matrix(c(0, 360), nrow(ltrb), 2, byrow = TRUE),
                    use_center = TRUE,
                    ...,
                    props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], nrow(ltrb))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], nrow(ltrb))
  }
  validate_length(
    nrow(ltrb),
    nrow(rsx_trans),
    nrow(angle),
    length(width),
    ncol(color)
  )

  sk_draw_arc(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    t(ltrb[, 1:4]),
    matrix(0, nrow(ltrb), 2),
    use_center,
    t(angle[, 1:2]),
    t(rsx_trans[, 1:6]),
    width,
    as.integer(color)
  )
}
