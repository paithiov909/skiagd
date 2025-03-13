#' img and props
#'
#' @rdname skiagd-params
#' @name param-img-and-props
#' @param img A raw vector of picture.
#' @param props A paint properties out of [paint()].
#' @keywords internal
NULL

#' Applying affine transformations to a picture
#'
#' @description
#' When loading a picture into a canvas,
#' you can apply an affine transformation
#' by providing a numeric vector of length 9 to [paint()] as `transform`.
#'
#' @details
#' This vector defines a transformation matrix that modifies a picture
#' before rendering it onto the canvas.
#'
#' The `transform` vector represents a 3x3 matrix
#' used for affine transformations, following the format:
#'
#' \deqn{
#' \begin{bmatrix} scale_x & skew_x & trans_x \\ skew_y & scale_y & trans_y \\ pers_0 & pers_1 & pers_2 \end{bmatrix}
#' }
#'
#' The first two rows define standard affine transformations,
#' including scaling, skewing, and translation.
#' The third row (`pers_0`, `pers_1`, and `pers_2`) is
#' typically used for perspective transformations,
#' though in most affine transformations, it remains as `[0, 0, 1]`.
#'
#' @seealso
#' * [Matrix in skia_safe::matrix - Rust](https://rust-skia.github.io/doc/skia_safe/matrix/struct.Matrix.html)
#' @rdname transform-matrix
#' @name transform-matrix
NULL
