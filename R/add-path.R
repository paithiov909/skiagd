#' Add paths
#'
#' @param path Characters of SVG notations
#' like `"M10 10 H 90 V 90 H 10 L 10 10"`.
#' @param rsx_trans A double matrix where each row represents an RSX transform.
#' Each column of the matrix corresponds to the scale, the angle of rotation,
#' the amount of translation
#' in the X-axis direction and in the Y-axis direction,
#' and the X and Y coordinates of the pivot point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_path <- function(img, path, rsx_trans,
                     ...,
                     props = paint()) {
  dots <- rlang::list2(...)
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], length(path))
  }
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], length(path))
  }
  validate_length(nrow(rsx_trans), width)

  sk_draw_path(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    path,
    t(rsx_trans),
    width,
    as.integer(color),
    props[["fill_type"]]
  )
}
