#' Create a canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill A string; color name.
#' @param size An integer; canvas size.
#' @returns A raw vector of picture.
#' @export
canvas <- function(fill = paint()[["color"]], size = paint()[["canvas_size"]]) {
  sk_absolute_fill(size, col2rgba(fill))
}

#' Add points
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_point <- function(img, point, props = paint()) {
  if (any(!is.finite(point))) {
    rlang::abort("point must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_draw_points(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    point[, 1],
    point[, 2],
    props[["point_mode"]]
  )
}

#' Add lines
#'
#' @param from A double matrix where each row is a start point.
#' @param to A double matrix where each row is an end point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_line <- function(img, from, to, props = paint()) {
  if (any(!is.finite(c(from, to)))) {
    rlang::abort("from and to must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_draw_line(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    from[, 1],
    from[, 2],
    to[, 1],
    to[, 2]
  )
}

#' Add circles
#'
#' @param center A double matrix where each row is circle center.
#' @param radius A double vector of circle radius.
#' @inheritParams param-img-and-props
#' @returns A raw vector picture.
#' @export
add_circle <- function(img, center, radius, props = paint()) {
  if (any(!is.finite(c(center, radius)))) {
    rlang::abort("x, y, and r must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_draw_circle(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    center[, 1],
    center[, 2],
    radius
  )
}

#' Add rectangles
#'
#' @param rect A double matrix where each row is a rectangle corner.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_rect <- function(img, rect, props = paint(), .size = dev_size()) {
  if (any(!is.finite(rect))) {
    rlang::abort("rect must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_draw_irect(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    rect[, 1],
    rect[, 2],
    rect[, 3],
    rect[, 4]
  )
}

#' Add paths
#'
#' @param path A character vector of SVG strings.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_path <- function(img, path, props = paint()) {
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_draw_path(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_props(props),
    path
  )
}

#' Convert picture into PNG data
#'
#' @inheritParams param-img-and-props
#' @returns A raw vector of PNG data.
#' @export
as_png <- function(img, props = paint()) {
  sk_as_png(props[["canvas_size"]], img, props[["transform"]])
}

#' Plot picture as PNG image
#'
#' @inheritParams param-img-and-props
#' @export
draw_img <- function(img, props = paint()) {
  if (requireNamespace("magick", quietly = TRUE)) {
    img <- as_png(img, props)
    png <- grDevices::as.raster(magick::image_read(img))
    plot(png)
  } else {
    rlang::abort("magick package is required")
  }
}
