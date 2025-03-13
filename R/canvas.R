#' Create a canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill A string; color name.
#' @param size An integer; canvas size.
#' @returns A raw vector of picture.
#' @export
canvas <- function(fill = paint()[["color"]], size = paint()[["canvas_size"]]) {
  sk_absolute_fill(size, col2rgba(fill))
}
