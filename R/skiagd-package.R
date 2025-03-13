## usethis namespace: start
#' @importFrom rlang env_get
#' @useDynLib skiagd, .registration = TRUE
## usethis namespace: end
#' @keywords internal
"_PACKAGE"

check_finite <- function(...) {
  if (any(!is.finite(c(...)))) {
    rlang::abort("coordinates must be finite values")
  }
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
  withr::with_options(
    list(.skiagd_paint_group = paint(...)),
    expr
  )
}
