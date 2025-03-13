## usethis namespace: start
#' @useDynLib skiagd, .registration = TRUE
#' @importFrom rlang env_get
## usethis namespace: end
#' @keywords internal
"_PACKAGE"

#' Color to RGBA
#'
#' A wrapper of [grDevices::col2rgb()].
#' In general, you don't need to use this function explicitly.
#'
#' @param color `col` for [grDevices::col2rgb()].
#' @returns an integer vector.
#' @export
col2rgba <- function(color) {
  as.vector(grDevices::col2rgb(color, alpha = TRUE))[1:4]
}

#' Device size
#'
#' Just returns the size of the current device as an integer (not a numeric).
#'
#' @param units `units` for [grDevices::dev.size()].
#' @returns an integer vector.
#' @export
dev_size <- function(units = "px") {
  as.integer(grDevices::dev.size(units))
}

#' Paint group
#'
#' Evaluates `expr` with [paint(...)].
#'
#' @param expr Expressions.
#' @param ... Any other arguments are passed to [paint()].
#' @export
with_group <- function(expr, ...) {
  # TODO: check if this can be nested
  withr::with_options(list(.skiagd_paint_group = paint(...)),
    expr
  )
}
