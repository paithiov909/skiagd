#' Create new canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill RGBA representation of a color.
#' This can be specified using named colors or hexadecimal color codes,
#' which are converted internally using [grDevices::col2rgb()].
#' @param canvas_size Integers of length 2.
#' @returns A raw vector of picture.
#' @export
canvas <- function(
  fill = "transparent",
  canvas_size = paint()[["canvas_size"]]
) {
  if (!is.numeric(fill)) {
    fill <- col2rgba(fill)
  }
  sk_absolute_fill(canvas_size, fill)
}
