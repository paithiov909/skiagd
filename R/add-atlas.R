#' Add atlas
#'
#' Draws a PNG sprite multiple times (as an atlas) onto an existing picture.
#'
#' @details
#' The number of sprites drawn is determined by `nrow(rsx_trans)`. Each row of
#' `rsx_trans` specifies an RSX transform applied to the sprite (scale, rotation,
#' translation, and anchor offsets).
#'
#' @note
#' * To create sprites (or canvases) of a specific size,
#' you may need to fix the size of the current graphics device (e.g., using a dummy device)
#' or supply `canvas_size` via `props = paint(canvas_size = ...)`,
#' because drawing functions use `props[["canvas_size"]]` at each call.
#'
#' @param png A raw vector of a PNG image to be used as a sprite. This can be
#'  created by [as_png()] from another picture, or read from a `.png` file using
#'  [readBin()].
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' # To fix the canvas size to 48x16, open a dummy device
#' png(nullfile(), width = 48, height = 16)
#'
#' # Create a simple arrow sprite (48x16) as a PNG
#' arrow_png <-
#'   canvas("transparent") |>
#'   add_line(
#'     from = matrix(c(4, 8), ncol = 2),
#'     to = matrix(c(44, 8), ncol = 2)
#'   ) |>
#'   add_line(
#'     from = matrix(c(
#'       36, 4,
#'       44, 8,
#'       36, 12
#'     ), ncol = 2, byrow = TRUE),
#'     to = matrix(c(
#'       44, 8,
#'       36, 12,
#'       44, 8
#'     ), ncol = 2, byrow = TRUE)
#'   ) |>
#'   as_png()
#'
#' # Close the dummy device
#' dev.off()
#'
#' # Place the sprite with different rotations and positions
#' # (anchor offsets set to the sprite center: 24, 8)
#' rsx_trans <-
#'   dplyr::tibble(
#'     sc = 1,
#'     rot = c(0, pi / 6, pi / 3, pi / 2, pi * 3 / 4),
#'     tx = c(100, 200, 300, 200, 100),
#'     ty = c(100, 100, 100, 200, 200),
#'     ax = 24,
#'     ay = 8
#'   )
#'
#' # Open a dummy device again
#' png(nullfile(), width = 400, height = 300)
#'
#' img <-
#'   canvas("white") |>
#'   add_atlas(arrow_png, rsx_trans = rsx_trans)
#'
#' # Close the dummy device
#' dev.off()
#'
#' # Draw the picture
#' draw_img(img)
#' }
add_atlas <- function(img, png, rsx_trans, ..., props = paint()) {
  sk_draw_atlas(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    t(rsx_trans[, 1:6, drop = FALSE])
  )
}
