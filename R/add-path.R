#' Add paths
#'
#' @param path A character vector of SVG notations
#' like `M10 10 H 90 V 90 H 10 L 10 10`.
#' @param transform A numeric vector of length 9
#' to apply affine transformations to the path themselves.
#' See [transform-matrix] for affine transformations.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_path <- function(img, path,
                     transform = c(1, 0, 0, 0, 1, 0, 0, 0, 1),
                     props = paint()) {
  props <- getOption(".skiagd_paint_group") %||% props
  sk_draw_path(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    path,
    transform,
    props[["fill_type"]]
  )
}
