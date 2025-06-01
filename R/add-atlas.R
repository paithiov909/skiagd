#' Add atlas
#'
#' @description
#' Draws the same number of sprites as the number of rows in `rsx_trans`.
#'
#' @param img A raw vector of picture.
#' @param png A raw vector of PNG image to be used as a sprite.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_atlas <- function(img, png, rsx_trans, ..., props = paint()) {
  sk_draw_atlas(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    t(rsx_trans[, 1:6])
  )
}
