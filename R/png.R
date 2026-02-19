#' Add PNG image to canvas
#'
#' Draws a PNG image onto an existing picture.
#'
#' @inheritParams param-img-and-props
#' @param png A raw vector of a PNG image.
#' @param left A numeric scalar giving the horizontal offset (in pixels) of the
#'  PNG's top-left corner from the canvas origin. Negative values are allowed.
#' @param top A numeric scalar giving the vertical offset (in pixels) of the
#'  PNG's top-left corner from the canvas origin. Negative values are allowed.
#' @returns A raw vector containing a serialized picture.
#' @export
add_png <- function(img, png, left = 0, top = 0, ..., props = paint()) {
  sk_draw_png(
    props[["canvas_size"]],
    img,
    props[["transform"]],
    as_paint_attrs(props),
    png,
    as.integer(c(left, top))
  )
}

#' Convert picture into PNG image
#'
#' Renders a serialized picture to a PNG image.
#'
#' @inheritParams param-img-and-props
#' @returns A raw vector of a PNG image.
#' @export
#' @examples
#' \dontrun{
#' png <-
#'  canvas("navy") |>
#'  as_png()
#'
#' # Write the PNG image to a file
#' writeBin(png, "navy.png")
#' }
as_png <- function(img, ..., props = paint()) {
  sk_as_png(props[["canvas_size"]], img, props[["transform"]])
}

#' Freeze picture
#'
#' @description
#' Rasterizes a picture into a PNG and adds it to a new canvas.
#'
#' This is equivalent to `as_png(img, props = props)` and then drawing the
#' resulting PNG onto a fresh canvas using [add_png()] with the default blend mode
#' (`BlendMode$SrcOver`). It can be used to reduce the number of recorded drawing
#' operations in a picture.
#'
#' @param left A numeric scalar giving the horizontal offset (in pixels) where the
#'  rasterized image is drawn on the new canvas. Negative values are allowed.
#' @param top A numeric scalar giving the vertical offset (in pixels) where the
#'  rasterized image is drawn on the new canvas. Negative values are allowed.
#' @param fill An RGBA color specification for the background fill of the new canvas.
#'  You can provide a named color or a hexadecimal color code, which is converted
#'  using [col2rgba()].
#' @inheritParams param-img-and-props
#' @returns A raw vector containing a serialized picture.
#' @export
freeze <- function(
  img,
  left = 0,
  top = 0,
  fill = "transparent",
  ...,
  props = paint()
) {
  img |>
    as_png(props = props) |>
    add_png(
      canvas(fill = fill, canvas_size = props[["canvas_size"]]),
      png = _,
      left = left,
      top = top
    )
}

.mime_map <- c(
  png = "image/png",
  jpg = "image/jpeg",
  jpeg = "image/jpeg",
  gif = "image/gif",
  webp = "image/webp",
  avif = "image/avif"
)
.supported_ext <- c("png", "jpg", "jpeg", "gif", "webp", "avif")

#' Embed an image as HTML
#'
#' @description
#' Embeds an image file as a base64-encoded HTML `<img>` tag.
#'
#' For string input, the image file is read from disk,
#' encoded as a data URI, and wrapped in an HTML object.
#' For raw input, the image data is directly embedded in an HTML object,
#' assuming it is a valid PNG image data.
#'
#' @details
#' This function is designed as a lightweight helper for visualization and
#' documentation purposes. It does not perform any image decoding itself and
#' relies on the input being readable by `base64enc::dataURI()`.
#'
#' @param x File path or raw image data.
#' @param max_size Maximum allowed file size in bytes.
#'  If `NULL`, size checking is disabled.
#' @param browsable Logical.
#'  Whether to wrap the result in `htmltools::browsable()`.
#' @param ... Additional attributes passed to `htmltools::tags$img()`.
#' @returns
#'  * `embed_img()` returns an HTML object.
#'  * `data_uri()` returns a data URI string.
#' @export
embed_img <- function(
  x,
  max_size = 4 * 1024^2,
  browsable = TRUE,
  ...
) {
  if (!requireNamespace("htmltools", quietly = TRUE)) {
    cli::cli_abort("You need `htmltools` to use this function.")
  }
  dat <- data_uri(x, max_size, ...)
  htmltools::browsable(
    htmltools::tags$img(
      src = dat,
      style = paste(
        "display:block;",
        "max-width:100%;",
        "height:auto;",
        "margin:1em auto;",
        "padding:0.5em;"
      ),
      ...
    ),
    value = browsable
  )
}

#' @rdname embed_img
#' @export
data_uri <- function(
  x,
  max_size = 4 * 1024^2,
  ...
) {
  UseMethod("data_uri")
}

#' @export
data_uri.raw <- function(
  x,
  max_size = 4 * 1024^2,
  ...
) {
  if (!requireNamespace("base64enc", quietly = TRUE)) {
    cli::cli_abort("You need `base64enc` to use this function.")
  }
  if (!is.null(max_size)) {
    size <- utils::object.size(x)
    if (is.na(size) || size > max_size) {
      cli::cli_abort(
        "File size ({format(size, standard = 'IEC')}) exceeds `max_size`."
      )
    }
  }
  base64enc::dataURI(x, mime = "image/png")
}

#' @export
data_uri.character <- function(
  x,
  max_size = 4 * 1024^2,
  ...
) {
  if (!requireNamespace("base64enc", quietly = TRUE)) {
    cli::cli_abort("You need `base64enc` to use this function.")
  }
  if (!file.exists(x)) {
    cli::cli_abort("{filename} does not exist.")
  }

  ext <- tolower(tools::file_ext(x))
  if (!nzchar(ext) || !ext %in% .supported_ext) {
    cli::cli_abort("Unsupported image format: .{ext}")
  }

  if (!is.null(max_size)) {
    size <- file.info(x)$size
    if (is.na(size) || size > max_size) {
      cli::cli_abort(
        "File size ({format(size, big.mark = ',')} bytes) exceeds `max_size`."
      )
    }
  }
  mime <- .mime_map[[ext]]
  base64enc::dataURI(file = x, mime = mime)
}
