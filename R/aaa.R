#' img and props
#'
#' @rdname skiagd-params
#' @name param-img-and-props
#' @param img A raw vector of picture.
#' @param props A paint properties out of [paint()].
#' @keywords internal
NULL # TODO: explain about picture

#' @export
print.PathEffect <- function(x, ...) {
  cat("PathEffect::", x$get_label(), "\n", sep = "")
}

#' @export
print.Shader <- function(x, ...) {
  cat("Shader::", x$get_label(), "\n", sep = "")
}

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
#' \deqn{\begin{bmatrix}
#' \text{scale}_x & \text{skew}_y & \text{pers}_0 \\
#' \text{skew}_x & \text{scale}_y & \text{pers}_1 \\
#' \text{trans}_x & \text{trans}_y & \text{pers}_2
#' \end{bmatrix}}
#'
#' The first two columns define standard affine transformations,
#' including scaling, skewing, and translation.
#' The third column (`pers_0`, `pers_1`, and `pers_2`) is
#' typically used for perspective transformations,
#' though in most affine transformations, it remains as `[0, 0, 1]`.
#'
#' @seealso
#' * [Matrix in skia_safe::matrix - Rust](https://rust-skia.github.io/doc/skia_safe/matrix/struct.Matrix.html)
#' @rdname transform-matrix
#' @name transform-matrix
NULL
