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
#' and `text_width` to get widths of textblobs.
#'
#' @details
#' Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#'
#' @param text Characters to be drawn.
#' @param point `NULL` or a double matrix where each row is the point
#' at which each character in `text` is drawn.
#' For example, if `text` is a character vector of 5 and 3 length strings,
#' `point` must be a 8x2 matrix.
#' If `NULL`, `text` is drawn at `c(0, props[["fontsize"]])` naturally.
#' @inheritParams param-img-and-props
#' @returns For `add_text()`, a raw vector of picture.
#' @export
add_text <- function(img, text, point = NULL, props = paint()) {
  if (anyNA(text)) {
    rlang::abort("`text` cannot contain NA.")
  }
  if (is.null(point)) {
    add_text_impl(img, text, props)
  } else {
    add_textblob_impl(img, text, point, props)
  }
}

add_text_impl <- function(img, text, props = paint()) {
  sk_draw_text(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text
  )
}

add_textblob_impl <- function(img, text, point, props = paint()) {
  if (sum(nchar(text)) != nrow(point)) {
    rlang::abort("Total number of characters in `text` and number of rows in `point` must be the same.")
  }
  sk_draw_textblob(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    point[, 1],
    point[, 2]
  )
}

#' @rdname add_text
#' @export
text_layout_horizontal <- function(text, props = paint()) {
  n_chars <- sum(nchar(text))
  vec <- c(
    seq(props[["fontsize"]] / 4, props[["fontsize"]] * n_chars, by = props[["fontsize"]]),
    rep_len(props[["fontsize"]], n_chars)
  )
  matrix(vec, ncol = 2)
}

#' @rdname add_text
#' @export
text_layout_vertical <- function(text, props = paint()) {
  n_chars <- sum(nchar(text))
  vec <- c(
    rep_len(props[["fontsize"]], n_chars),
    seq(props[["fontsize"]] / 4, props[["fontsize"]] * n_chars, by = props[["fontsize"]])
  )
  matrix(vec, ncol = 2)
}

#' @rdname add_text
#' @export
text_width <- function(text, props = paint()) {
  sk_get_text_width(
    text,
    as_paint_attrs(props)
  )
}
