#' Transform SVG paths
#'
#' @param path Characters of SVG notations.
#' @param transform Numerics of length 9.
#' @returns A character vector of SVG notations.
#' @export
#' @family path-utils
svg_transform <- function(path, transform) {
  sk_path_transform(path, transform)
}

#' Interpolate between two SVG paths
#'
#' Interpolates between two SVG paths of equal length.
#' If the paths are not interpolatable, an error is thrown.
#'
#' @param t A numeric vector of weights in range `(0, 1)`.
#' If it is inbetween or outside the range, the path is interpolated.
#' @param first A string scalar of SVG notation.
#' @param second A string scalar of SVG notation.
#' @returns A character vector of SVG notations interpolated.
#' @export
#' @family path-utils
svg_interpolate <- function(t, first, second) {
  sk_path_interpolate(t, first, second)
}

#' Retrieve bounding boxes of SVG paths
#'
#' @param path Characters of SVG notations.
#' @returns A tibble.
#' @export
#' @family path-utils
svg_bounds <- function(path) {
  ret <- sk_path_bounds(path) |>
    as.data.frame()
  ret[["id"]] <- ret[["id"]] + 1
  class(ret) <- c("tbl_df", "tbl", "data.frame")
  ret
}
