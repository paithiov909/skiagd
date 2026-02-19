#' Add arcs
#'
#' Adds one or more elliptical arcs to an existing picture.
#'
#' @details
#' The number of arcs is determined by `nrow(ltrb)`. The number of rows of
#' `rsx_trans` and `angle`, and the lengths of `sigma` and `width` (and the number
#' of columns of `color`) must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all arcs.
#'
#' When `use_center = TRUE` and the sweep in `angle` describes a full circle
#' (e.g., `c(0, 360)`), the geometry includes the wedge edges from the oval's
#' rightmost point toward the center. Depending on `style`, this may be filled
#' and thus not visually apparent.
#'
#' @note
#' * Arcs are drawn from an oval specified by a rounded rectangle internally,
#' and this geometry does not support rotation.
#' If the rotation angle in `rsx_trans` is not zero, this function will error.
#' To draw rotated arcs or more general shapes, use [add_path()] instead.
#'
#' @param ltrb A numeric matrix (or a data-frame-like object)
#'  with 4 numeric columns (left, top, right, bottom),
#'  where each row defines the bounding box of the oval that contains the arc.
#' @param angle A numeric matrix (or a data-frame-like object)
#'  with 2 numeric columns giving the sweep angles (in degrees) for each arc.
#' @param use_center A logical value indicating whether to draw the wedge edges,
#'  i.e., lines from the oval center to the arc end points (like a "Pac-Man" wedge).
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' # Arrange arcs by shifting coordinates in `ltrb`
#' arcs <-
#'  dplyr::tibble(
#'    l = c(60, 260, 60),
#'    t = c(60, 60, 240),
#'    r = c(220, 420, 220),
#'    b = c(200, 200, 380),
#'    angle_start = c(0, 45, 0),
#'    angle_end = c(270, 270, 360)
#'  )
#'
#' canvas("white") |>
#'   add_arc(
#'     ltrb = dplyr::select(arcs, l, t, r, b),
#'     angle = dplyr::select(arcs, angle_start, angle_end),
#'     use_center = TRUE
#'   ) |>
#'   draw_img()
#'
#' # Arrange arcs by translating the same `ltrb` via `rsx_trans`
#' arcs <-
#'   dplyr::tibble(
#'     l = 60,
#'     t = 60,
#'     r = 220,
#'     b = 200,
#'     angle_start = c(0, 0, 0),
#'     angle_end = c(270, 180, 360)
#'   )
#' rsx_trans <-
#'   dplyr::tibble(
#'      sc = 1,
#'      rot = 0,
#'      tx = c(0, 200, 0),
#'      ty = c(0, 0, 180),
#'      ax = 0,
#'      ay = 0
#'   )
#'
#' canvas("white") |>
#'   add_arc(
#'     ltrb = dplyr::select(arcs, l, t, r, b),
#'     rsx_trans = rsx_trans,
#'     angle = dplyr::select(arcs, angle_start, angle_end),
#'     use_center = TRUE
#'   ) |>
#'   draw_img()
#' }
add_arc <- function(
  img,
  ltrb,
  rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), nrow(ltrb), 6, byrow = TRUE),
  angle = matrix(c(0, 360), nrow(ltrb), 2, byrow = TRUE),
  use_center = TRUE,
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
    nrow(angle),
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_arc(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    t(ltrb[, 1:4, drop = TRUE]),
    matrix(0, nrow(ltrb), 2),
    use_center,
    t(angle[, 1:2, drop = TRUE]),
    t(rsx_trans[, 1:6, drop = TRUE]),
    sigma,
    width,
    as.integer(color)
  )
}
