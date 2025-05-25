#' Add paths
#'
#' @param path Characters of SVG notations
#' like `"M10 10 H 90 V 90 H 10 L 10 10"`.
#' @param transform Numerics of length 9
#' to apply affine transformations to the path themselves.
#' See [transform-matrix] for affine transformations.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_path <- function(img, path, transform = diag(1, 3),
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
  validate_length(length(width), path)

  sk_draw_path(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    path,
    width,
    as.integer(color),
    transform,
    props[["fill_type"]]
  )
}
