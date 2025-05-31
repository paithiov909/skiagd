#' Add text
#'
#' @description
#' Draws text as textblobs.
#'
#' The return value is often a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always [freeze()] the picture after drawing text.
#'
#' You can use `text_layout_horizontal()` and `text_layout_vertical()`
#' to create a `point` matrix
#' and `text_width()` to get widths of textblobs.
#'
#' @details
#' Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#'
#' @param text Characters to be drawn.
#' @param point `NULL` or a double matrix where each row is the point
#' at which each character in `text` is drawn.
#' For example, if `text` is a character vector of 5 and 3 length strings,
#' `point` must contain 8 points.
#' If `NULL`, `text` is drawn at `c(0, props[["fontsize"]])` naturally.
#' @inheritParams param-img-and-props
#' @returns For `add_text()`, a raw vector of picture.
#' @export
add_text <- function(img, text, rsx_trans, ..., props = paint()) {
  if (anyNA(text)) {
    rlang::abort("`text` cannot contain NA.")
  }
  dots <- rlang::list2(...)
  color <- dots[["color"]]
  if (is.null(color)) {
    color <- rep(props[["color"]], length(text))
  }
  sk_draw_text(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    t(rsx_trans),
    color
  )
}

#' @rdname add_text
#' @export
text_width <- function(text, props = paint()) {
  sk_get_text_width(
    text,
    as_paint_attrs(props)
  )
}

#' @rdname add_text
#' @export
text_count <- function(text, props = paint()) {
  # TODO: implement
  # sk_get_text_count(
  #   text,
  #   as_paint_attrs(props)
  # )
}
