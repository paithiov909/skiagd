#' Add vertices
#'
#' @details
#' When `colors` is provided, they will be combined with
#' `props[["shader"]]` if present or opaque `props[["color"]]` if not.
#'
#' Therefore, if you want to paint each vertex with a different color,
#' you must specify a shader as `paint(shader = Shader$color(...))`.
#'
#' @param vertices A double matrix where each row is a point.
#' If `nrow(vertices)` is not a multiple of 3,
#' the last `nrow(vertices) %% 3` points are ignored.
#' @param colors An integer matrix where each ***column*** is an RGBA color
#' for each vertex. If `NULL`, all vertices are drawn with `props[["color"]]`.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_vertices <- function(img, vertices, colors = NULL, props = paint()) {
  vertices <- vertices[seq_len(nrow(vertices) - (nrow(vertices) %% 3)), ]
  if (rlang::is_empty(vertices)) {
    rlang::abort("Requires at least 3 vertices.")
  }
  if (is.null(colors)) {
    colors <- rep(props[["color"]], nrow(vertices))
  }
  sk_draw_vertices(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    vertices[, 1],
    vertices[, 2],
    as.integer(colors),
    props[["vertex_mode"]]
  )
}
