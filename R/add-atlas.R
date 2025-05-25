#' Add atlas
#'
#' @param img A raw vector of picture.
#' @param png A raw vector of PNG image to be used as the sprite.
#' @param rsx_trans A double matrix where each row represents an RSX transform.
#' Each column of the matrix corresponds to the scale, the angle of rotation,
#' the amount of translation
#' in the X-axis direction and in the Y-axis direction,
#' and the X and Y coordinates of the pivot point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_atlas <- function(img, png, rsx_trans, ..., props = paint()) {
  sk_draw_atlas(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    rsx_trans[, 1],
    rsx_trans[, 2],
    rsx_trans[, 3],
    rsx_trans[, 4],
    rsx_trans[, 5],
    rsx_trans[, 6]
  )
}
