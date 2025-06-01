#' Add text
#'
#' @note
#' * `rsx_trans` for `add_text()` must have
#' the same number of rows as the total number of characters to be drawn;
#' not just the length of `text`.
#' * Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#' * The return value is often a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always [freeze()] the picture after drawing text.
#'
#' @param text Characters to be drawn.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
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
  validate_length(
    length(text),
    ncol(color)
  )

  sk_draw_text(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    t(rsx_trans[, 1:6]),
    as.integer(color)
  )
}

#' Get width and number of characters
#'
#' Returns information about text strings
#' when they are drawn naturally as a textblob with `props`.
#'
#' @param text Text strings.
#' @param props A list of painting attributes out of [paint()].
#' @returns A data frame.
#' @export
text_info <- function(text, props = paint()) {
  ret <-
    sk_get_text_info(
      text,
      as_paint_attrs(props)
    )
  out <-
    data.frame(
      id = ret[["id"]] + 1L,
      n_chars = ret[["n_chars"]],
      width = ret[["width"]]
    )
  class(out) <- c("tbl_df", "tbl", "data.frame")
  out
}
