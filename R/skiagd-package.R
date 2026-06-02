## usethis namespace: start
#' @importFrom rlang env_get
## usethis namespace: end
#' @keywords internal
"_PACKAGE"

#' List available font families
#'
#' @description
#' Returns font families available on the system.
#'
#' Since skiagd can only access fonts installed on the system,
#' font families in the font registry or local fonts
#' registered by the [systemfonts](https://systemfonts.r-lib.org/) package
#' cannot be specified as the `family` in [paint()].
#'
#' @returns A tibble containing `family`.
#' @export
list_font_families <- function() {
  ret <- data.frame(
    family = sk_list_families()
  )
  class(ret) <- c("tbl_df", "tbl", "data.frame")
  ret
}
