#' Add text
#'
#' @note
#' * `rsx_trans` for `add_text()` must have
#' the same number of rows as the total number of characters to be drawn;
#' not just the length of `text`.
#' * Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#' * If `freeze` is `FALSE`, the return value would be a large object
#' because the specified font is embedded in the returned picture.
#' Note that you should almost always freeze the picture when drawing text.
#'
#' @param text Characters to be drawn.
#' @param freeze Whether to freeze the picture after drawing text.
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#' @returns A raw vector of picture.
#' @export
add_text <- function(
  img,
  text,
  rsx_trans,
  freeze = TRUE,
  ...,
  props = paint()
) {
  if (anyNA(text)) {
    cli::cli_abort("`text` cannot contain NA.")
  }
  dots <- rlang::list2(...)
  sigma <- dots[["sigma"]]
  if (is.null(sigma)) {
    sigma <- rep(props[["sigma"]], length(text))
  }
  color <- dots[["color"]]
  if (is.null(color) || !is_color_mat(color)) {
    color <- matrix(rep(props[["color"]], length(text)), nrow = 4)
  }
  validate_length(
    length(text),
    length(sigma),
    ncol(color)
  )

  sk_draw_text(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    text,
    freeze,
    t(rsx_trans[, 1:6]),
    sigma,
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
#' @returns A tibble containing `id`, `n_chars` and `width`.
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
