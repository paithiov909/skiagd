#' Add vertices
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_vertices <- function(img, vertices, props = paint()) {
  if (nrow(vertices) %% 3 != 0) {
    rlang::abort("The number of vertices must be a multiple of 3.")
  }
  sk_draw_vertices(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    vertices[, 1],
    vertices[, 2],
    props[["vertex_mode"]]
  )
}
