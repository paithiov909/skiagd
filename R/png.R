#' Add PNG image to canvas
#'
#' @inheritParams param-img-and-props
#' @param png A raw vector of PNG image.
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @returns A raw vector of picture.
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
#' @inheritParams param-img-and-props
#' @returns A raw vector of PNG image.
#' @export
as_png <- function(img, ..., props = paint()) {
  sk_as_png(props[["canvas_size"]], img, props[["transform"]])
}

#' Freeze picture
#'
#' `as_png(img, props)` and then adds it to a new canvas
#' with the default blend mode (`BlendMode$SrcOver`).
#'
#' @param left Left offset for drawing PNG image.
#' @param top Top offset for drawing PNG image.
#' @param fill RGBA representation of a color.
#'  This can be specified using named colors or hexadecimal color codes,
#'  which are converted internally using [colorfast::col_to_rgb()].
#' @inheritParams param-img-and-props
#' @returns A raw vector of picture.
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
#' The image file is read from disk, encoded as a data URI, and wrapped in a
#' browsable HTML object.
#'
#' @details
#' This function is designed as a lightweight helper for visualization and
#' documentation purposes. It does not perform any image decoding itself and
#' relies on the file being readable by `base64enc::dataURI()`.
#'
#' @param filename Path to an image file.
#' @param max_size Maximum allowed file size in bytes.
#'  If `NULL`, file size checking is disabled.
#' @param browsable Logical.
#'  Whether to wrap the result in `htmltools::browsable()`.
#' @param ... Additional attributes passed to `htmltools::tags$img()`.
#' @returns An HTML object representing an embedded image.
#' @export
embed_img <- function(
  filename,
  max_size = 4 * 1024^2,
  browsable = TRUE,
  ...
) {
  if (
    !requireNamespace("base64enc", quietly = TRUE) ||
      !requireNamespace("htmltools", quietly = TRUE)
  ) {
    cli::cli_abort("You need `base64enc` and `htmltools` to use this function.")
  }
  if (!file.exists(filename)) {
    cli::cli_abort("{filename} does not exist.")
  }

  ext <- tolower(tools::file_ext(filename))
  if (!nzchar(ext) || !ext %in% .supported_ext) {
    cli::cli_abort("Unsupported image format: .{ext}")
  }

  if (!is.null(max_size)) {
    size <- file.info(filename)$size
    if (is.na(size) || size > max_size) {
      cli::cli_abort(
        "File size ({format(size, big.mark = ',')} bytes) exceeds `max_size`."
      )
    }
  }
  mime <- .mime_map[[ext]]
  dat <- base64enc::dataURI(file = filename, mime = mime)

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
