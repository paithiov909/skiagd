## usethis namespace: start
#' @useDynLib skiagd, .registration = TRUE
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
#' Evaluates `expr` with `paint(...)`.
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

#' Paint props
#'
#' @param ... <[`dynamic-dots`][rlang::dyn-dots]> What these dots do.
#' @returns a list.
#' @export
paint <- function(...) {
  default_props <- unclass(grid::get.gpar())
  default_props[["blend_mode"]] <- 1 # NOTE: 0~27

  dots <- rlang::list2(...)
  props <-
    purrr::list_assign(
      default_props,
      !!!dots
    )
  list(
    col = col2rgba(props[["col"]]),
    fill = if (col2rgba(props[["fill"]])[4] == 0) 2 else 1,
    ljoin = switch(props[["linejoin"]],
      "round" = 1,
      "mitre" = 2,
      "bevel" = 3
    ),
    lend = switch(props[["lineend"]],
      "round" = 1,
      "butt" = 2,
      "square" = 3
    ),
    lty = 0L, # FIXME: not used.
    lwd = props[["lwd"]],
    lmiter = props[["linemitre"]],
    blend_mode = props[["blend_mode"]]
  )
}
