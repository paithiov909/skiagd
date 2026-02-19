#' Add rectangles
#'
#' Adds one or more rounded rectangles to an existing picture.
#'
#' @details
#' The number of rectangles is determined by `nrow(ltrb)`. The number of rows of
#' `rsx_trans` and `radii`, and the lengths of `sigma` and `width`
#' (and the number of columns of `color`) must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all rectangles.
#'
#' @note
#' * Skia's rounded rectangle does not support rotation. If the rotation
#' angle in `rsx_trans` is not zero, this function will error. To draw rotated
#' rectangles or more general shapes, use [add_path()] instead.
#'
#' @param ltrb A numeric matrix (or a data-frame-like object)
#'  with 4 columns (left, top, right, bottom),
#'  where each row represents a rectangle.
#' @param radii A numeric matrix (or data-frame-like object)
#'  with 2 columns (x and y),
#'  where each row represents the corner radii of a rounded rectangle.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' # Arrange rectangles by shifting coordinates in `ltrb`
#' ltrb <-
#'   dplyr::tibble(
#'     l = c(60, 220, 60),
#'     t = c(60, 60, 180),
#'     r = c(180, 340, 180),
#'     b = c(140, 140, 260)
#'   )
#'
#' canvas("white") |>
#'   add_rect(
#'     ltrb = ltrb,
#'     radii = matrix(c(12, 12), nrow(ltrb), 2, byrow = TRUE)
#'   ) |>
#'   draw_img()
#'
#' # Arrange rectangles by translating the same `ltrb` via `rsx_trans`
#' ltrb <-
#'   dplyr::tibble(
#'     l = rep_len(60, 3),
#'     t = rep_len(60, 3),
#'     r = rep_len(180, 3),
#'     b = rep_len(140, 3)
#'   )
#' rsx_trans <-
#'   dplyr::tibble(
#'     sc = 1,
#'     rot = 0,
#'     tx = c(0, 160, 0),
#'     ty = c(0, 0, 120),
#'     ax = 0,
#'     ay = 0
#'   )
#'
#' canvas("white") |>
#'   add_rect(
#'     ltrb = ltrb,
#'     rsx_trans = rsx_trans,
#'     radii = matrix(c(12, 12), nrow(ltrb), 2, byrow = TRUE)
#'   ) |>
#'   draw_img()
#' }
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
    t(ltrb[, 1:4, drop = TRUE]),
    radii[, 1, drop = TRUE],
    radii[, 2, drop = TRUE],
    t(rsx_trans[, 1:6, drop = TRUE]),
    sigma,
    width,
    as.integer(color)
  )
}

#' Add difference rectangles
#'
#' Adds one or more difference rectangles (outer minus inner), optionally with
#' rounded corners.
#'
#' @details
#' The number of shapes is determined by `nrow(outer)`. The numbers of rows of
#' `inner`, `rsx_trans`, `outer_radii`, and `inner_radii`,
#' and the lengths of `sigma` and `width` (and the number of columns of `color`) must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all shapes.
#'
#' @note
#' * As with [add_rect()], Skia's rounded rectangle does not support rotation.
#' If the rotation angle in `rsx_trans` is not zero, this function will error.
#' To draw rotated rectangles or more general shapes, use [add_path()].
#'
#' @param outer,inner A numeric matrix (or a data-frame-like object)
#'  with 4 numeric columns (left, top, right, bottom),
#'  where each row represents a rectangle.
#' @param outer_radii,inner_radii A numeric matrix (or data-frame-like object)
#'  with 2 numeric columns (x and y),
#'  where each row represents the corner radii of a rounded rectangle.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' # Shift rectangles by modifying coordinates directly
#' ltrb <-
#'   dplyr::tibble(
#'     lo = c(60, 280),
#'     to = c(60, 60),
#'     ro = c(240, 460),
#'     bo = c(200, 200),
#'     li = c(100, 320),
#'     ti = c(100, 100),
#'     ri = c(200, 420),
#'     bi = c(160, 160)
#'   )
#'
#' canvas("white") |>
#'   add_diff_rect(
#'     outer = dplyr::select(ltrb, lo, to, ro, bo),
#'     inner = dplyr::select(ltrb, li, ti, ri, bi),
#'     outer_radii = matrix(c(16, 16), nrow(ltrb), 2, byrow = TRUE),
#'     inner_radii = matrix(c(8, 8), nrow(ltrb), 2, byrow = TRUE)
#'   ) |>
#'   draw_img()
#'
#' # Use the same geometry and translate via `rsx_trans`
#' ltrb <-
#'   dplyr::tibble(
#'     lo = rep_len(60, 2),
#'     to = rep_len(60, 2),
#'     ro = rep_len(240, 2),
#'     bo = rep_len(200, 2),
#'     li = rep_len(100, 2),
#'     ti = rep_len(100, 2),
#'     ri = rep_len(200, 2),
#'     bi = rep_len(160, 2)
#'   )
#' rsx_trans <-
#'   dplyr::tibble(
#'     sc = 1,
#'     rot = 0,
#'     tx = c(0, 220),
#'     ty = c(0, 0),
#'     ax = 0,
#'     ay = 0
#'   )
#'
#' canvas("white") |>
#'   add_diff_rect(
#'     outer = dplyr::select(ltrb, lo, to, ro, bo),
#'     inner = dplyr::select(ltrb, li, ti, ri, bi),
#'     rsx_trans = rsx_trans
#'   ) |>
#'   draw_img()
#' }
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
    t(outer[, 1:4, drop = TRUE]),
    outer_radii[, 1, drop = TRUE],
    outer_radii[, 2, drop = TRUE],
    t(inner[, 1:4, drop = TRUE]),
    inner_radii[, 1, drop = TRUE],
    inner_radii[, 2, drop = TRUE],
    t(rsx_trans[, 1:6, drop = TRUE]),
    sigma,
    width,
    as.integer(color)
  )
}
