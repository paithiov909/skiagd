#' Transform SVG path
#'
#' @param path Characters of SVG notations.
#' @param transform Numerics of length 9.
#' @returns For `path_transform()`: a character vector of SVG notations.
#' @export
#' @rdname skiagd-path-utils
path_transform <- function(path, transform) {
  sk_path_transform(path, transform)
}

#' Interpolate between two SVG paths
#'
#' Interpolates between two SVG paths of equal length.
#' If the paths are not interpolatable, an error is thrown.
#'
#' @param t A numeric vector of weights.
#' If it is inbetween or outside the range, the path is interpolated.
#' @param first A string scalar of SVG notation.
#' @param second A string scalar of SVG notation.
#' @returns For `path_interpolate()`: a character vector of SVG notations interpolated.
#' @export
#' @rdname skiagd-path-utils
path_interpolate <- function(t, first, second) {
  sk_path_interpolate(t, first, second)
}

#' Retrieve SVG path bounds
#'
#' Returns minimum and maximum axes values of `path`.
#'
#' @param path Characters of SVG notations.
#' @returns For `path_bounds()`: a numeric matrix.
#' @export
#' @rdname skiagd-path-utils
path_bounds <- function(path) {
  ret <- sk_path_size(path)
  dim(ret) <- c(length(path), 2)
  ret
}
