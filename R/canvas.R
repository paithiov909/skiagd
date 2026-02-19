#' Create a new canvas
#'
#' @description
#' Creates a new serialized Skia picture (a single frame) with a canvas filled
#' with the specified color.
#'
#' The coordinate system follows the usual image convention: the origin is at
#' the top-left, the X axis increases to the right, and the Y axis increases
#' downward. Units are pixels.
#'
#' @param fill An RGBA color specification for the background fill.
#'  You can also provide a named color or a hexadecimal color code, which is
#'  converted internally using [colorfast::col_to_rgb()].
#' @param canvas_size An integer vector of length 2 specifying canvas width and height, in pixels.
#' @returns A raw vector containing a serialized Skia picture.
#' @export
#' @examples
#' \dontrun{
#' canvas("navy") |> draw_img()
#' canvas(c(255, 0, 0, 255)) |> draw_img() # filled with red
#' }
canvas <- function(
  fill = "transparent",
  canvas_size = paint()[["canvas_size"]]
) {
  if (!is.numeric(fill)) {
    fill <- col2rgba(fill)
  }
  sk_absolute_fill(canvas_size, fill)
}
