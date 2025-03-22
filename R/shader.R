#' @export
print.Shader <- function(x, ...) {
  cat("Shader::", x$get_label(), "\n", sep = "")
}

#' Shader
#'
#' `Shader` is a struct that offers a reference to `skia_safe::Shader`.
#' You can apply a shader to paintings via [paint()].
#'
#' @seealso
#' * [Gradients | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/gradients)
#' * [Perlin Noise Shaders | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/perlin-noise)
#' * [Blending and Colors | React Native Skia](https://shopify.github.io/react-native-skia/docs/shaders/colors)
#' @rdname skiagd-shader
#' @name Shader
NULL
