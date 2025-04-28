#' Add points
#'
#' @param point A double matrix where each row is a point.
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
#' @export
add_point <- function(img, point, props = paint()) {
  if (!inherits(props, "paint_attrs")) {
    purrr::reduce(seq_along(props), \(curr, i) {
      add_point(
        curr,
        point[i, , drop = FALSE],
        props = props[[i]]
      ) |>
        freeze(props = props[[i]])
    }, .init = img)
  } else {
    sk_draw_points(
      props[["canvas_size"]],
      img,
      props[["transform"]],
      as_paint_attrs(props),
      point[, 1],
      point[, 2],
      props[["point_mode"]]
    )
  }
}
