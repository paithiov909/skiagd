#' @export
print.PathEffect <- function(x, ...) {
  cat("PathEffect::", x$get_label(), "\n", sep = "")
}

#' PathEffect
#'
#' `PathEffect` is a struct that offers a reference to `skia_safe::PathEffect`.
#' You can apply a path effect to drawings via [paint()].
#' Currently only single `PathEffect` can be specified; multiple effects are not supported.
#'
#' @details
#' The following effects are available:
#'
#' * `no_effect()`: does not apply any path effect. This is the default effect for [paint()].
#' * `trim(start, end)`: trims the `start` and `end` of the path.
#' * `discrete(lentgh, deviation, seed)`: applies discrete path effect.
#' * `dash(intervals, phase)`: applies dash path effect.
#' * `corner(radius)`: applies corner path effect.
#' * `path_1d(path, advance, phase, style)`: applies 1D path effect.
#' * `path_2d(path, transform)`: applies 2D path effect.
#' * `line_2d(width, transform)`: applies 2D line path effect.
#'
#' @param start A numeric scalar in the range `[0, 1]`.
#' @param end A numeric scalar in the range `[0, 1]`.
#' @param length A numeric scalar; length of the subsegments.
#' @param deviation A numeric scalar; limit of the movement of the endpoints.
#' @param seed An integer scalar; random seed.
#' @param intervals A numeric vector; even number of entries with even indices
#' specifying the length of the "on" intervals,
#' and the odd index specifying the length of "off".
#' @param phase A numeric scalar; offset into the intervals array (for `dash()`),
#' or distance (mod advance) along the path for its initial position (for `path_1d()`).
#' @param radius A numeric scalar; radius of the rounded corners.
#' @param path A string scalar of SVG notation to replicate.
#' @param advance A numeric scalar; space between instances of path.
#' @param style A string scalar; how to transform path at each point.
#' Can be `"translate"`, `"rotate"`, or `"morph"`.
#' @param transform Numerics of length 9; transformation matrix.
#' @param width A numeric scalar; width of the path to be stamped.
#'
#' @returns A `PathEffect` object.
#' @seealso
#' [Path Effects | React Native Skia](https://shopify.github.io/react-native-skia/docs/path-effects/)
#' @family paint-attributes
#' @rdname skiagd-path-effect
#' @name PathEffect
NULL
