#' Create transformation matrix from points pairs
#'
#' Creates a transformation matrix from two sets of points.
#' If mapping is not possible, throws an error.
#'
#' @param src A double matrix where each row is a point.
#' @param dst A double matrix where each row is a point.
#' @returns A numeric matrix of size 3x3.
#' @export
create_mapping <- function(src, dst) {
  ret <-
    sk_matrix_map_point(
      src[, 1],
      src[, 2],
      dst[, 1],
      dst[, 2]
    )
  matrix(ret, ncol = 3)
}
