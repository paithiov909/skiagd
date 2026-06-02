#' Params for drawing functions
#'
#' @rdname skiagd-params
#' @name param-img-and-props
#' @param img A raw vector of a serialized picture.
#' @param props A list of painting attributes created by [paint()].
#' @param ... For some drawing functions, you can specify
#'  `sigma`, `width`, and `color` as named arguments.
#'
#' * `sigma` must be a numeric vector of blur sigmas for each shape.
#' * `width` must be a numeric vector of stroke widths for each shape.
#' * `color` must be an integer matrix with 4 rows (RGBA) and N columns (shapes).
#'
#'  If they are not provided as named arguments, they will be taken from `props`.
#'  If the function does not matter them,  `...` is simply ignored.
#' @keywords internal
NULL

#' RSX transform
#'
#' @rdname skiagd-params
#' @name param-rsx-trans
#' @param rsx_trans A numeric matrix (or a data-frame-like object)
#'  with 6 columns where each row represents an RSX transform.
#'  Each column of the matrix corresponds to:
#'
#' * scale
#' * angle of rotation (in radians)
#' * amount of translation in the X-axis direction
#' * amount of translation in the Y-axis direction
#' * offset for the anchor point in the X-axis direction
#' * offset for the anchor point in the Y-axis direction
#' @keywords internal
NULL

#' Pictures
#'
#' @description
#' In Skia, a picture is a prerecorded list of drawing operations on a canvas.
#' The drawing functions of skiagd
#' take it as their first argument, add new shapes onto it,
#' and return a serialized picture as a raw vector again.
#'
#' A serialized picture is a binary format containing a single frame,
#' which can be saved to a `.skp` file using [writeBin()],
#' and reused by any drawing functions
#' as long as it is compatible with the version of Skia
#' used to create it.
#'
#' You can review contents of `.skp` files
#' with the [Skia debugger](https://skia.org/docs/dev/tools/debugger/)
#' if they are compatible with the version.
#'
#' @seealso
#' [Pictures | React Native Skia](https://shopify.github.io/react-native-skia/docs/shapes/pictures)
#' @rdname pictures
#' @name pictures
#' @aliases picture
NULL

#' Affine transformation matrix
#'
#' @description
#' Several skiagd APIs accept `transform` as numerics of length 9.
#' This value is interpreted as a 3x3 affine transformation matrix.
#'
#' Typical uses include transforming:
#'
#' * shader coordinate systems such as gradients and image shaders,
#' * path effect patterns such as `PathEffect$path_2d()` and `PathEffect$line_2d()`,
#' * SVG path data via [svg_transform()].
#'
#' @details
#' The matrix is read in the following layout:
#'
#' \deqn{\begin{bmatrix}
#' \text{scale}_x & \text{skew}_y & \text{persp}_0 \\
#' \text{skew}_x & \text{scale}_y & \text{persp}_1 \\
#' \text{trans}_x & \text{trans}_y & \text{persp}_2
#' \end{bmatrix}}
#'
#' For ordinary affine transforms, the last column is typically `c(0, 0, 1)`,
#' while the other entries control scaling, skewing, and translation.
#'
#' skiagd passes this matrix through to the corresponding Skia API, so the exact
#' effect depends on where `transform` is used.
#'
#' @seealso
#' [Matrix in skia_safe::matrix - Rust](https://rust-skia.github.io/doc/skia_safe/matrix/struct.Matrix.html)
#' @rdname transform-matrix
#' @name transform-matrix
NULL
