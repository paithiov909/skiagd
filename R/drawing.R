#' Create a canvas
#'
#' Creates a new canvas filled with specified color.
#'
#' @param fill A string; color name.
#' @inheritParams param-dot-size
#' @returns A raw vector.
#' @export
canvas <- function(fill = paint()[["fill"]], .size = dev_size()) {
  sk_absolute_fill(.size, col2rgba(fill))
}

#' Add points
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @inheritParams param-dot-size
#' @returns A raw vector.
#' @export
add_point <- function(img, point, mode = c("points", "lines", "polygon"), props = paint(), .size = dev_size()) {
  if (any(!is.finite(point))) {
    rlang::abort("point must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  mode <- rlang::arg_match(mode)
  mode <- switch(mode,
    "points" = 0L,
    "lines" = 1L,
    "polygon" = 2L
  )
  sk_points(.size, img, point, props, mode)
}

#' Add lines
#'
#' @param from A double matrix where each row is a start point.
#' @param to A double matrix where each row is an end point.
#' @inheritParams param-img-and-props
#' @inheritParams param-dot-size
#' @returns a raw vector.
#' @export
add_line <- function(img, from, to, props = paint(), .size = dev_size()) {
  if (any(!is.finite(c(from, to)))) {
    rlang::abort("from and to must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_line(.size, img, from, to, props)
}

#' Add circles
#'
#' @param center A double matrix where each row is circle center.
#' @param radius A double vector of circle radius.
#' @inheritParams param-img-and-props
#' @inheritParams param-dot-size
#' @returns A raw vector.
#' @export
add_circle <- function(img, center, radius, props = paint(), .size = dev_size()) {
  if (any(!is.finite(c(center, radius)))) {
    rlang::abort("x, y, and r must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_circle(.size, img, center, radius, props)
}

#' Add rectangles
#'
#' @param rect A double matrix where each row is a rectangle corner.
#' @inheritParams param-img-and-props
#' @inheritParams param-dot-size
#' @returns A raw vector.
#' @export
add_rect <- function(img, rect, props = paint(), .size = dev_size()) {
  if (any(!is.finite(rect))) {
    rlang::abort("rect must be finite values")
  }
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_irect(.size, img, rect, props)
}

#' Add paths
#'
#' @param path A character vector of SVG strings.
#' @param translate An integer vector length 2.
#' @param scale A double vector length 2.
#' @inheritParams param-img-and-props
#' @inheritParams param-dot-size
#' @returns A raw vector.
#' @export
add_path <- function(img, path, translate = c(0L, 0L), scale = c(1L, 1L), props = paint(), .size = dev_size()) {
  if (!is.null(paint_group <- getOption(".skiagd_paint_group"))) {
    props <- paint_group
  }
  sk_svg_path(.size, img, path, as.integer(translate), as.integer(scale), props)
}

#' Save image to file
#'
#' @param img A raw vector.
#' @param filename A string; file name to save.
#' @inheritParams param-dot-size
#' @returns `filename` is invisibly returned.
#' @export
save_img <- function(img, filename, .size = dev_size()) {
  invisible(sk_save_png(.size, img, filename))
}

#' Plot image
#'
#' @param img A raw vector.
#' @export
draw_img <- function(img) {
  if (requireNamespace("magick", quietly = TRUE)) {
    png <- grDevices::as.raster(magick::image_read(img))
    plot(png)
  } else {
    stop("magick package is required")
  }
}
