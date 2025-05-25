#' Params for drawing functions
#'
#' @rdname skiagd-params
#' @name param-img-and-props
#' @param img A raw vector of picture.
#' @param props A list of painting attributes out of [paint()].
#' @param ... For some drawing functions, you can specify `width` and `color` as named arguments.
#'
#' * `width` must be a numeric vector of stroke widths for each shape.
#' * `color` must be an integer matrix where each ***column*** is an RGBA color for each shape.
#'
#' If they are not provided as named arguments, they will be taken from `props`.
#' If the function does not matter them,  `...` is simply ignored.
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
#' @section Notes:
#' * A serialized picture can hold only up to 100 drawing operations.
#' If you need more, you can save the capacity by using [freeze()] as needed.
#'
#' @seealso
#' [Pictures | React Native Skia](https://shopify.github.io/react-native-skia/docs/shapes/pictures)
#' @rdname pictures
#' @name pictures
#' @aliases picture
NULL

#' Applying affine transformations to the previous picture
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
#' though in most affine transformations, it remains as `c(0, 0, 1)`.
#'
#' @seealso
#' * [Matrix in skia_safe::matrix - Rust](https://rust-skia.github.io/doc/skia_safe/matrix/struct.Matrix.html)
#' @rdname transform-matrix
#' @name transform-matrix
NULL
