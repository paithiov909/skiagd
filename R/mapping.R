#' Create transformation matrix from points pairs
#'
#' Creates a transformation matrix from sets of points `src` and `dst`.
#' If mapping is not possible, throws an error.
#'
#' @param src A numeric matrix (or a data-frame-like object)
#'  with 2 columns (x and y) and 4 rows,
#'  where each row is a point.
#' @param dst A numeric matrix (or a data-frame-like object)
#'  with 2 columns (x and y) and 4 rows,
#'  where each row is a point.
#' @returns A numeric matrix of size 3x3.
#' @export
#' @examples
#' \dontrun{
#' cv_size <- dev_size()
#'
#' # Create an affine matrix that fits a 768x576 picture to current device size
#' create_mapping(
#'   src = matrix(
#'     c(0, 0, 768, 0, 768, 576, 0, 576),
#'   ),
#'   dst = matrix(
#'     c(0, 0, cv_size[1], 0, cv_size[1], cv_size[2], 0, cv_size[2]),
#'     ncol = 2,
#'     byrow = TRUE
#'   )
#' )
#' }
create_mapping <- function(src, dst) {
  ret <-
    sk_matrix_map_point(
      src[1:4, 1, drop = TRUE],
      src[1:4, 2, drop = TRUE],
      dst[1:4, 1, drop = TRUE],
      dst[1:4, 2, drop = TRUE]
    ) |>
    matrix(, ncol = 3)
  # for compatibility with 'affiner' package
  class(ret) <- c("transform2d", "at_matrix", class(ret))
  ret
}
