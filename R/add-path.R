#' Add paths
#'
#' @param path Characters of SVG notations like `"M10 10 H 90 V 90 H 10 L 10 10"`.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_path <- function(img, path,
                     rsx_trans = matrix(c(1, 0, 0, 0, 0, 0), length(path), 6, byrow = TRUE),
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
  validate_length(
    length(path),
    nrow(rsx_trans),
    length(width),
    ncol(color)
  )

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
