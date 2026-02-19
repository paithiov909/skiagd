#' Add vertices
#'
#' Adds a vertex mesh to an existing picture.
#'
#' @details
#' The vertex mode is taken from `props[["vertex_mode"]]`. The blur sigma is taken
#' from `props[["sigma"]]`.
#'
#' Vertex colors are per-vertex attributes. If `color` is supplied via `...`, it
#' must be an RGBA integer matrix with 4 rows and `nrow(vertices)` columns (one
#' color per vertex). If `color` is not supplied, `props[["color"]]` is recycled
#' to all vertices.
#'
#' @note
#' When vertex colors are present, the paint's blend source is determined by:
#' * `props[["shader"]]` if a shader is set, otherwise
#' * an opaque version of `props[["color"]]`.
#'
#' This source is then combined with the interpolated vertex colors. As a result,
#' to paint each vertex with a different visible color, you typically need to set
#' a [Shader] (see the examples below).
#'
#' @param vertices A numeric matrix (or a data-frame-like object)
#'  with 2 numeric columns (x and y),
#'  where each row is a vertex position.
#'  Vertices are consumed in groups of three (three rows per triangle).
#'  If `nrow(vertices)` is not a multiple of 3, the last `nrow(vertices) %% 3` vertices are ignored.
#' @inheritParams param-img-and-props
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' # A single triangle with per-vertex colors.
#' # To make the vertex colors visible as-is, set a shader.
#' canvas("white") |>
#'   add_vertices(
#'     dplyr::tibble(
#'       x = c(128, 0, 128),
#'       y = c(256, 256, 0)
#'     ),
#'     color = col2rgba(c("#61dafb", "#fb61da", "#dafb61")),
#'     props = paint(
#'       shader = Shader$from_picture(
#'         canvas("#ffffff00"),
#'         TileMode$Repeat,
#'         dev_size(),
#'         diag(3)
#'       )
#'     )
#'   ) |>
#'   draw_img()
#' }
add_vertices <- function(img, vertices, ..., props = paint()) {
  vertices <- vertices[seq_len(nrow(vertices) - (nrow(vertices) %% 3)), ]
  if (rlang::is_empty(vertices)) {
    cli::cli_abort("Requires at least 3 vertices.")
  }
  dots <- rlang::list2(...)
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], nrow(vertices)), nrow = 4)
  }
  validate_length(
    nrow(vertices),
    ncol(color)
  )

  sk_draw_vertices(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    vertices[, 1, drop = TRUE],
    vertices[, 2, drop = TRUE],
    props[["sigma"]],
    as.integer(color),
    props[["vertex_mode"]]
  )
}
