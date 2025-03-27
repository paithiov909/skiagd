#' Create new canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill RGBA representation of a color.
#' This can be specified using named colors or hexadecimal color codes,
#' which are converted internally using [grDevices::col2rgb()].
#' @param size Integers of length 2; canvas size.
#' @returns A raw vector of picture.
#' @export
canvas <- function(fill = "transparent", size = paint()[["canvas_size"]]) {
  if (!is.numeric(fill)) {
    fill <- col2rgba(fill)
  }
  sk_absolute_fill(size, fill)
}
