#' @export
print.PathEffect <- function(x, ...) {
  cat("PathEffect::", x$get_label(), "\n", sep = "")
}

#' @export
c.PathEffect <- function(...) {
  purrr::reduce(list(...), function(acc, nxt) {
    PathEffect$sum(acc, nxt)
  })
}

#' PathEffect
#'
#' @description
#' `PathEffect` is a struct that offers a reference to `skia_safe::PathEffect`.
#' You can apply a path effect to shapes via [paint()].
#'
#' Concatenating path effects with `c()` is equivalent to sum them sequentially
#' into a single effect using `PathEffect$sum()`.
#'
#' @details
#' The following effects are available:
#'
#' * `no_effect()`: does not apply any path effect. This is the default effect for [paint()].
#' * `sum(first, second)`: applies two effects in sequence.
#' * `trim(start, end)`: trims the `start` and `end` of the path. Note that you can't trim nothing at all, i.e., setting `start = 0` and `end = 1` does nothing.
#' * `discrete(lentgh, deviation, seed)`: applies discrete path effect.
#' * `dash(intervals, phase)`: applies dash path effect.
#' * `corner(radius)`: applies corner path effect.
#' * `path_1d(path, advance, phase, style)`: applies 1D path effect.
#' * `path_2d(path, transform)`: applies 2D path effect.
#' * `line_2d(width, transform)`: applies 2D line path effect.
#'
#' @param first A `PathEffect` object.
#' @param second A `PathEffect` object.
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
