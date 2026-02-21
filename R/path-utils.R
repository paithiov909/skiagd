#' Transform SVG paths
#'
#' @description
#' Applies an affine transformation to SVG path notations.
#'
#' This is useful for translating, scaling, or skewing paths written in the
#' SVG `d` attribute syntax before placing them with [add_path()].
#'
#' @param path A character vector of SVG path notations (the `d` attribute syntax).
#'  Each element is treated as a separate path.
#' @param transform Numerics of length 9 representing a [transform-matrix].
#'  This defines a 3x3 matrix for affine transformation.
#' @returns A character vector of transformed SVG path notations.
#' @export
#' @family path-utils
#' @examples
#' trans <- matrix(c(1, 0, -50, 0, 1, -50, 0, 0, 1), 3, 3)
#' svg_transform("M45 10 H55 V45 H90 V55 H55 V90 H45 V55 H10 V45 H45 Z", trans)
svg_transform <- function(path, transform) {
  sk_path_transform(path, transform)
}

#' Interpolate between two SVG paths
#'
#' Interpolates between two SVG paths that are compatible for morphing.
#'
#' This function returns intermediate paths between `first` and `second` using
#' weights `t`. Paths must be interpolatable (e.g., compatible command sequences);
#' otherwise an error is thrown.
#'
#' @param t A numeric vector of interpolation weights. Values between 0 and 1
#'  produce intermediate paths; values outside this range are wrapped.
#' @param first A string scalar of an SVG path notation (the `d` attribute syntax).
#' @param second A string scalar of an SVG path notation (the `d` attribute syntax).
#' @returns A character vector of SVG path notations interpolated.
#' @export
#' @family path-utils
#' @examples
#' trans <- matrix(c(1, 0, -50, 0, 1, -50, 0, 0, 1), 3, 3)
#' first <- svg_transform(R"(
#'  M10 18 H40 M18 10 V40
#'  M90 18 H60 M82 10 V40
#'  M10 82 H40 M18 90 V60
#'  M90 82 H60 M82 90 V60
#' )", trans)
#' second <- svg_transform(R"(
#'  M12 22 H44 M22 12 V44
#'  M88 22 H56 M78 12 V44
#'  M12 78 H44 M22 88 V56
#'  M88 78 H56 M78 88 V56
#' )", trans)
#' svg_interpolate(seq(-2, 2, length.out = 10), first, second)
svg_interpolate <- function(t, first, second) {
  sk_path_interpolate(t, first, second)
}

#' Retrieve bounding boxes of SVG paths
#'
#' Computes axis-aligned bounding boxes for one or more SVG paths.
#'
#' @param path A character vector of SVG path notations (the `d` attribute syntax).
#'  Each element is treated as a separate path.
#' @returns
#' A tibble containing columns `id`, `left`, `top`, `right` and `bottom` for each path.
#' `id` is a 1-based index corresponding to the input order of `path`.
#' @export
#' @family path-utils
#' @examples
#' svg_bounds("M45 10 H55 V45 H90 V55 H55 V90 H45 V55 H10 V45 H45 Z")
svg_bounds <- function(path) {
  ret <- sk_path_bounds(path) |>
    as.data.frame()
  ret[["id"]] <- ret[["id"]] + 1
  class(ret) <- c("tbl_df", "tbl", "data.frame")
  ret
}
