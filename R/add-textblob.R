#' Add text
#'
#' @description
#' Draws text as textblobs.
#'
#' The return value is often a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always [freeze()] the picture after drawing text.
#'
#' @details
#' You can use `text_layout_horizontal()` and `text_layout_vertical()`
#' to create a `point` matrix.
#'
#' @param text Strings to be drawn.
#' @param point A double matrix where each row is the point
#' at which each character in `text` is drawn.
#' For example, if `text` is a character vector of 5 and 3 strings,
#' `point` must be a 8x2 matrix.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_text <- function(img, text,
                     point = text_layout_horizontal(text, props),
                     props = paint()) {
  if (anyNA(text)) {
    rlang::abort("`text` cannot contain NA.")
  }
  if (sum(nchar(text)) != nrow(point)) {
    rlang::abort("Number of characters in `text` and number of rows in `point` must be the same.")
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
