#' Add SVG paths
#'
#' @description
#' Adds one or more SVG paths to an existing picture.
#'
#' Each path is given as a string in the SVG `d`-attribute syntax, and can be
#' positioned by providing an RSX transform per path via `rsx_trans`.
#'
#' @details
#' The number of paths is determined by `length(path)`. The number of rows of
#' `rsx_trans`, and the lengths of `sigma` and `width` (and the number of columns
#' of `color`) must match this number.
#'
#' The drawing attributes `sigma`, `width`, and `color` can be supplied via `...`.
#' If they are not supplied, they are taken from `props` and recycled to all paths.
#'
#' The [FillType] for closed paths is taken from `props[["fill_type"]]`
#' and applied to all paths.
#'
#' @param path A character vector of SVG path notations (the `d` attribute syntax),
#'  e.g. `"M45 10 H55 V45 H90 V55 H55 V90 H45 V55 H10 V45 H45 Z"`.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector containing a serialized picture with the added paths.
#' @export
#' @examples
#' \dontrun{
#' # Single path
#' canvas("white") |>
#'   add_path("M45 10 H55 V45 H90 V55 H55 V90 H45 V55 H10 V45 H45 Z") |>
#'   draw_img()
#'
#' # Place the same path multiple times using `rsx_trans`
#' path <- rep("M45 10 H55 V45 H90 V55 H55 V90 H45 V55 H10 V45 H45 Z", 3)
#' rsx_trans <-
#'   dplyr::tibble(
#'     sc = 1,
#'     rot = 0,
#'     tx = c(0, 120, 0),
#'     ty = c(0, 0, 120),
#'     ax = 1,
#'     ay = 1
#'   )
#'
#' canvas("white") |>
#'   add_path(path, rsx_trans = rsx_trans) |>
#'   draw_img()
#' }
add_path <- function(
  img,
  path,
  rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), length(path), 6, byrow = TRUE),
  ...,
  props = paint()
) {
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], length(path))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], length(path))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], length(path)), nrow = 4)
  }
  validate_length(
    length(path),
    nrow(rsx_trans),
    length(sigma),
    length(width),
    ncol(color)
  )

  sk_draw_path(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    path,
    t(rsx_trans[, 1:6, drop = TRUE]),
    sigma,
    width,
    as.integer(color),
    props[["fill_type"]]
  )
}
