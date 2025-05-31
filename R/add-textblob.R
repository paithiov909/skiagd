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
#' Since textblobs do not have font fallback mechanism,
#' characters out of the specified font are not drawn correctly.
#'
#' @param text Characters to be drawn.
#' @param rsx_trans A double matrix where each row represents an RSX transform.
#' Each column of the matrix corresponds to the scale, the angle of rotation,
#' the amount of translation
#' in the X-axis direction and in the Y-axis direction,
#' and the X and Y coordinates of the pivot point.
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
