#' Add lines
#'
#' @param from A double matrix where each row is a start point.
#' @param to A double matrix where each row is an end point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_line <- function(img, from, to, props = paint()) {
  if (!inherits(props, "paint_attrs")) {
    purrr::reduce(seq_along(props), \(curr, i) {
      add_line(
        curr,
        from[i, , drop = FALSE],
        to[i, , drop = FALSE],
        props = props[[i]]
      )
    }, .init = img)
  } else {
    sk_draw_line(
      props[["canvas_size"]],
      img,
      props[["transform"]],
      as_paint_attrs(props),
      from[, 1],
      from[, 2],
      to[, 1],
      to[, 2]
    )
  }
}
