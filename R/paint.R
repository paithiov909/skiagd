#' Define painting attributes
#'
#' @description
#' The `paint()` function allows users to specify
#' various painting attributes for drawing shapes on the canvas,
#' such as color, stroke width, and transformations.
#'
#' @param ... <[`dynamic-dots`][rlang::dyn-dots]>
#' Named arguments specifying painting attributes. See details.
#'
#' @details
#' The following painting attributes can be specified:
#'
#' * `canvas_size`: Integers of length 2 (width, height).
#' * `color`: An RGBA color specification (integers of length 4). This can be also specified using named colors or hexadecimal color codes, which are converted using [col2rgba()].
#' * `style`: Paint style. See [Style].
#' * `join`: Stroke join. See [Join].
#' * `cap`: Stroke cap. See [Cap].
#' * `width`: A numeric scalar (stroke width).
#' * `miter`: A numeric scalar (stroke miter).
#' * `fontsize`: A numeric scalar (font size).
#' * `family`: Font family name. You can list available font families using [list_font_families()].
#' * `fontface`: Font face. See [FontStyle].
#' * `sigma`: A numeric scalar. Default value for blur sigma.
#' * `blur_style`: [BlurStyle] for a blur mask filter applied to the shape.
#' * `blend_mode`: See [BlendMode].
#' * `path_effect`: See [PathEffect].
#' * `shader`: See [Shader].
#' * `image_filter`: See [ImageFilter].
#' * `point_mode`: [PointMode] for [add_point()].
#' * `vertex_mode`: [VertexMode] for [add_vertices()].
#' * `fill_type`: [FillType] for [add_path()].
#' * `transform`: Numerics of length 9. See [transform-matrix] for affine transformations.
#'
#' @returns A list containing the specified painting attributes,
#'  merged with default values.
#' @export
paint <- function(...) {
  dots <- rlang::list2(...)
  if (all(!is.null(dots[["color"]]), !is.numeric(dots[["color"]]))) {
    dots[["color"]] <- col2rgba(dots[["color"]])
  }
  ret <-
    purrr::list_assign(
      default_attrs(),
      !!!dots
    )
  class(ret) <- c("paint_attrs", class(ret))
  ret
}

#' Get the size of the current graphics device
#'
#' Just returns the size of the current graphics device as integers.
#'
#' @param units `units` for [grDevices::dev.size()].
#' @returns An integer vector of length 2 (width, height).
#' @export
dev_size <- function(units = "px") {
  as.integer(grDevices::dev.size(units))
}

#' Convert colors to a matrix of RGBA integers
#'
#' A wrapper of [colorfast::col_to_rgb()].
#'
#' @param color `col` for [colorfast::col_to_rgb()].
#' @returns An integer matrix with 4 rows (RGBA) and N columns (the same length as `color`).
#' @export
col2rgba <- function(color) {
  colorfast::col_to_rgb(color)
}

default_attrs <- function() {
  props <- unclass(grid::get.gpar())
  list(
    canvas_size = dev_size(),
    color = col2rgba(props[["col"]]),
    style = if (col2rgba(props[["fill"]])[4] == 0) {
      env_get(Style, "Stroke")
    } else {
      env_get(Style, "Fill")
    },
    join = switch(
      props[["linejoin"]],
      "round" = env_get(Join, "Round"),
      "mitre" = env_get(Join, "Miter"),
      "bevel" = env_get(Join, "Bevel")
    ),
    cap = switch(
      props[["lineend"]],
      "round" = env_get(Cap, "Round"),
      "butt" = env_get(Cap, "Butt"),
      "square" = env_get(Cap, "Square")
    ),
    width = props[["lwd"]],
    miter = props[["linemitre"]],
    fontsize = props[["fontsize"]],
    family = if (props[["fontfamily"]] == "") {
      "sans"
    } else {
      props[["fontfamily"]]
    },
    fontface = switch(
      as.character(props[["font"]]),
      "1" = env_get(FontStyle, "Normal"),
      "2" = env_get(FontStyle, "Bold"),
      "3" = env_get(FontStyle, "Italic"), # "italic" or "oblique"
      "4" = env_get(FontStyle, "BoldItalic"),
      "5" = env_get(FontStyle, "Normal"), # "cyrillic"
      "6" = env_get(FontStyle, "Italic"), # "cyrillic.oblique"
      "7" = env_get(FontStyle, "Normal") # "EUC"
    ),
    sigma = 0,
    blur_style = env_get(BlurStyle, "Normal"),
    blend_mode = env_get(BlendMode, "SrcOver"),
    path_effect = PathEffect$no_effect(),
    shader = Shader$no_shader(),
    image_filter = ImageFilter$no_filter(),
    point_mode = env_get(PointMode, "Points"),
    vertex_mode = env_get(VertexMode, "Triangles"),
    fill_type = env_get(FillType, "Winding"),
    transform = diag(1, 3)
  )
}

as_paint_attrs <- function(p) {
  PaintAttrs$set_attrs(
    p[["color"]],
    p[["style"]],
    p[["join"]],
    p[["cap"]],
    p[["width"]],
    p[["miter"]],
    p[["fontsize"]],
    p[["family"]],
    p[["fontface"]],
    p[["blend_mode"]],
    p[["blur_style"]],
    p[["path_effect"]],
    p[["shader"]],
    p[["image_filter"]]
  )
}
