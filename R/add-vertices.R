#' Add vertices
#'
#' @param vertices A double matrix where each row is a point.
#' If `nrow(vertices)` is not a multiple of 3,
#' the last `nrow(vertices) %% 3` points are ignored.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_vertices <- function(img, vertices, props = paint()) {
  vertices <- vertices[seq_len(nrow(vertices) - (nrow(vertices) %% 3)), ]
  if (rlang::is_empty(vertices)) {
    rlang::abort("Requires at least 3 vertices.")
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
