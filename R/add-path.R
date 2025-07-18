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
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], length(path))
  }
  width <- dots[["width"]]
  if (is.null(width)) {
    width <- rep(props[["width"]], length(path))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], length(path)), nrow = 4)
  }
  validate_length(
    length(path),
    nrow(rsx_trans),
    length(sigma),
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
    sigma,
    width,
    as.integer(color),
    props[["fill_type"]]
  )
}
