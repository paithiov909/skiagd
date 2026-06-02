# Lock environments so that users cannot modify them
# even if they are assigned to the global environment.
lockEnvironment(Style, bindings = TRUE)
lockEnvironment(Join, bindings = TRUE)
lockEnvironment(Cap, bindings = TRUE)
lockEnvironment(FontStyle, bindings = TRUE)
lockEnvironment(BlurStyle, bindings = TRUE)
lockEnvironment(BlendMode, bindings = TRUE)
lockEnvironment(PathEffect, bindings = TRUE)
lockEnvironment(Shader, bindings = TRUE)
lockEnvironment(ImageFilter, bindings = TRUE)
lockEnvironment(PointMode, bindings = TRUE)
lockEnvironment(VertexMode, bindings = TRUE)
lockEnvironment(FillType, bindings = TRUE)
lockEnvironment(RuntimeEffect, bindings = TRUE)
lockEnvironment(TileMode, bindings = TRUE)


#' Enable autocomplete for painting attributes
#'
#' Enables autocomplete for painting attributes.
#' To do this, this function simply assigns them to the current environment.
#'
#' @param env Environment to assign objects to.
#' @returns Called for its side-effect.
#' @export
enable_autocomplete <- function(env = parent.frame()) {
  objs <- c(
    "Style",
    "Join",
    "Cap",
    "FontStyle",
    "BlurStyle",
    "BlendMode",
    "PathEffect",
    "Shader",
    "ImageFilter",
    "PointMode",
    "VertexMode",
    "FillType",
    "RuntimeEffect",
    "TileMode"
  )
  ns <- asNamespace("skiagd")
  for (nm in objs) {
    assign(nm, get(nm, envir = ns), envir = env)
  }
  invisible(NULL)
}
