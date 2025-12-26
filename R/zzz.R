.onUnload <- function(libpath) {
  library.dynam.unload("skiagd", libpath)
}

#' Log execution time of an expression
#'
#' An operator to evaluate an expression
#' while logging its execution time for debugging.
#'
#' @details
#' `lhs %timer% rhs` evaluates `rhs` inside a [system.time] call,
#' assigns the execution time to the variable `time`,
#' and tries to evaluate `lhs` in an environment where `time` exists.
#' In doing so, failure to evaluate the left-hand side
#' does not result in an error.
#'
#' @param lhs An expression.
#' @param rhs An expression.
#' @returns Values from evaluated `rhs` is returned invisibly.
#' @export
#' @examples
#' print(time) %timer% {
#'  rnorm(10) ^2
#' }
`%timer%` <- function(lhs, rhs) {
  lhs <- rlang::enquo(lhs)
  rhs <- rlang::enquo(rhs)
  time <- system.time({
    ret <- rlang::eval_tidy(rhs)
  })
  rlang::try_fetch(
    rlang::eval_tidy(lhs, data = list(time = time)),
    error = function(e) {
      rlang::warn("Failed to evaluate lhs.", parent = e)
    }
  )
  invisible(ret)
}

#' Check that all arguments have the expected length
#'
#' Aborts the call if any argument does not have the expected length.
#'
#' @param ... Numeric vectors.
#' @noRd
validate_length <- function(expected, ...) {
  len <- c(...)
  if (!all(len == expected)) {
    rlang::abort(
      "Some arguments have different lengths than others.",
      call = rlang::caller_env()
    )
  }
  TRUE
}

#' Check if x looks like a color matrix
#'
#' @param x An object to be checked.
#' @returns A logical scalar.
#' @noRd
is_color_mat <- function(x) {
  ret <- is.matrix(x) && nrow(x) == 4
  if (!ret) {
    rlang::warn(
      "`color` does not seem to be a color matrix. Falling back to the default color of `paint()`.",
      call = rlang::caller_env()
    )
  }
  ret
}
