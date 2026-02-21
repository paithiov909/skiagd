#' Add text
#'
#' Draws text strings as text blobs.
#'
#' @param text A character vector of text strings to be drawn. `NA_character_` is not allowed.
#'  Each element of `text` is handled as one text blob.
#' @param freeze A logical value indicating whether to freeze the picture after drawing text.
#'  If `TRUE`, the result is rasterized and re-added to a new canvas (like [freeze()]).
#' @inheritParams param-img-and-props
#' @inheritParams param-rsx-trans
#'
#' @details
#' The placement of glyphs is controlled by `rsx_trans`. For `add_text()`,
#' `rsx_trans` must have the same number of rows as the total number of characters
#' to be drawn, i.e. `sum(nchar(text))`, not `length(text)`.
#'
#' In contrast, `sigma` and `color` provided via `...` (or from `props`) are matched
#' to `length(text)` (one value per text element). If you need per-character values
#' for `sigma` or `color`, split your string into single characters and pass them
#' as a character vector.
#'
#' @note
#' * Text blobs do not have a font fallback mechanism. Characters not supported by
#'   the specified font may not render correctly.
#' * If `freeze = FALSE`, the returned picture can become large because font data may
#'   be embedded. In most cases, it is recommended to keep `freeze = TRUE`.
#'
#' @returns A raw vector containing a serialized picture.
#' @export
#' @examples
#' \dontrun{
#' if ("Noto Sans Mono" %in% list_font_families()[["family"]]) {
#'   info <- text_info(
#'     "Hello, skiagd!",
#'     props = paint(family = "Noto Sans Mono", fontsize = 36)
#'   )
#'   offset <- seq(0, info$n_chars * (info$width / info$n_chars)
#'   rsx_trans <-
#'     dplyr::tibble(
#'       sc = 1,
#'       rot = 0,
#'       x = (dev_size()[1] - info$width) / 2 + offset, length.out = info$n_chars),
#'       y = dev_size()[2] / 2,
#'       ax = 0,
#'       ay = 0
#'     )
#'   canvas("white") |>
#'     add_text(
#'       "Hello, skiagd!",
#'       rsx_trans = rsx_trans,
#'       props = paint(family = "Noto Sans Mono", fontsize = 36)
#'     ) |>
#'     draw_img()
#' }
#' }
add_text <- function(
  img,
  text,
  rsx_trans,
  freeze = TRUE,
  ...,
  props = paint()
) {
  if (!is.character(text) || anyNA(text)) {
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
    t(rsx_trans[, 1:6, drop = TRUE]),
    sigma,
    as.integer(color)
  )
}

#' Get width, bounding box, and number of characters
#'
#' Returns metrics for text strings when they are shaped and drawn as a text blob
#' with the given `props`.
#'
#' @param text A character vector of text strings.
#' @param props A list of painting attributes created by [paint()].
#' @returns
#' A tibble containing one row per element of `text`, with columns:
#' `id` (1-based index), `n_chars`, `width` (advance width),
#' and `left`, `top`, `right`, `bottom` for the bounding box.
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
      width = ret[["advance_width"]],
      left = ret[["l"]],
      top = ret[["t"]],
      right = ret[["r"]],
      bottom = ret[["b"]]
    )
  class(out) <- c("tbl_df", "tbl", "data.frame")
  out
}
