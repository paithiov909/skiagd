#' Paint props
#'
#' @param ... <[`dynamic-dots`][rlang::dyn-dots]> What these dots do.
#' @returns a list.
#' @export
paint <- function(...) {
  dots <- rlang::list2(...)
  purrr::list_assign(
    default_props(),
    !!!dots
  )
}

default_props <- function() {
  props <- unclass(grid::get.gpar())
  list(
    canvas_size = dev_size(),
    color = col2rgba(props[["col"]]),
    style = if (col2rgba(props[["fill"]])[4] == 0) {
      env_get(Style, "Stroke")
    } else {
      env_get(Style, "Fill")
    },
    join = switch(props[["linejoin"]],
      "round" = env_get(Join, "Round"),
      "mitre" = env_get(Join, "Miter"),
      "bevel" = env_get(Join, "Bevel")
    ),
    cap = switch(props[["lineend"]],
      "round" = env_get(Cap, "Round"),
      "butt" = env_get(Cap, "Butt"),
      "square" = env_get(Cap, "Square")
    ),
    lty = 0,
    width = props[["lwd"]],
    miter = props[["linemitre"]],
    blend_mode = env_get(BlendMode, "Src"),
    point_mode = env_get(PointMode, "Points"),
    transform = sk_matrix_default()
  )
}

as_paint_props <- function(p) {
  PaintProps$set_props(
    p[["color"]],
    p[["style"]],
    p[["join"]],
    p[["cap"]],
    p[["lty"]],
    p[["width"]],
    p[["miter"]],
    p[["blend_mode"]]
  )
}
