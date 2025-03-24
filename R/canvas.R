#' Create a canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill A string scalar; named colors or hexadecimal color codes.
#' @param size Integers of length 2; canvas size.
#' @returns A raw vector of picture.
#' @export
canvas <- function(fill = "transparent", size = paint()[["canvas_size"]]) {
  sk_absolute_fill(size, col2rgba(fill))
}
