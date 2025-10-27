skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- grDevices::png(tempfile(fileext = ".png"), width = 720, height = 576)
on.exit(dev.off())

test_that("ImageFilter$color_matrix works", {
  vdiffr::expect_doppelganger(
    "color_matrix",
    canvas("transparent") |>
      add_rect(
        matrix(c(0, 0, dev_size()[1], dev_size()[2]), ncol = 4),
        props = paint(
          shader = Shader$turbulence(
            freq = c(0.01, 0.05),
            octaves = 2,
            seed = 2,
            tile_size = c(dev_size()[1], dev_size()[2])
          ),
          image_filter = ImageFilter$color_matrix(
            color_mat = matrix(
              # fmt: skip
              c(
                -.578, .990, .588, 0, 0,
                .469, .535, -.003, 0, 0,
                .015, 1.69, -.703, 0, 0,
                0, 0, 0, 1, 0
              ),
              ncol = 5,
              nrow = 4,
              byrow = TRUE
            )
          ),
        )
      ) |>
      as_recordedplot()
  )
})

test_that("ImageFilter$displacement_map works", {
  png <- system.file("images/lake.png", package = "skiagd")

  crop_rect <- c(0, 0, dev_size()[1], dev_size()[2])
  pict <-
    canvas("transparent") |>
    add_rect(
      matrix(crop_rect, ncol = 4),
      props = paint(
        shader = Shader$turbulence(
          freq = c(0.01, 0.05),
          octaves = 2,
          seed = 2,
          tile_size = c(dev_size()[1], dev_size()[2])
        )
      )
    )

  vdiffr::expect_doppelganger(
    "displacement_map",
    canvas("transparent") |>
      add_png(
        png = readBin(png, what = "raw", n = file.info(png)$size),
        # this sample image is 479x320
        left = 120,
        top = 128,
        props = paint(
          image_filter = ImageFilter$displacement_map(
            channels = c(1, 3),
            scale = 30,
            displacement = ImageFilter$from_picture(
              pict,
              matrix(crop_rect, ncol = 4)
            ),
            crop_rect = crop_rect
          )
        ),
      ) |>
      as_recordedplot()
  )
})

test_that("chromatic aberration looks good", {
  png <- system.file("images/lake.png", package = "skiagd")

  crop_rect <- c(0, 0, dev_size()[1], dev_size()[2])
  pict <-
    canvas("transparent") |>
    add_png(
      png = readBin(png, what = "raw", n = file.info(png)$size),
      # this sample image is 479x320
      left = 120,
      top = 128
    )

  vdiffr::expect_doppelganger(
    "chromatic_aberration",
    canvas("gray90") |>
      add_rect(
        matrix(crop_rect, ncol = 4),
        props = paint(
          shader = Shader$from_picture(
            pict,
            TileMode$Repeat,
            crop_rect[3:4],
            diag(3)
          ),
          image_filter = c(
            ImageFilter$color_matrix(
              matrix(
                # fmt: skip
                c(
                  0, 0, 0, 0, 0,
                  0, 1, 0, 0, 0, # Green
                  0, 0, 0, 0, 0,
                  0, 0, 0, .88, 0
                ),
                ncol = 5,
                nrow = 4,
                byrow = TRUE
              )
            ),
            ImageFilter$offset(c(-4, -4), crop_rect)
          ),
        )
      ) |>
      add_rect(
        matrix(crop_rect, ncol = 4),
        props = paint(
          shader = Shader$from_picture(
            pict,
            TileMode$Repeat,
            crop_rect[3:4],
            diag(3)
          ),
          image_filter = c(
            ImageFilter$color_matrix(
              matrix(
                # fmt: skip
                c(
                  0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0,
                  0, 0, 1, 0, 0, # Blue
                  0, 0, 0, .88, 0
                ),
                ncol = 5,
                nrow = 4,
                byrow = TRUE
              )
            ),
            ImageFilter$offset(c(4, 0), crop_rect)
          ),
          blend_mode = BlendMode$Lighten,
        )
      ) |>
      add_rect(
        matrix(crop_rect, ncol = 4),
        props = paint(
          shader = Shader$from_picture(
            pict,
            TileMode$Repeat,
            crop_rect[3:4],
            diag(3)
          ),
          image_filter = c(
            ImageFilter$color_matrix(
              matrix(
                # fmt: skip
                c(
                  1, 0, 0, 0, 0, # Red
                  0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0,
                  0, 0, 0, .88, 0
                ),
                ncol = 5,
                nrow = 4,
                byrow = TRUE
              )
            ),
            ImageFilter$offset(c(0, 4), crop_rect)
          )
        ),
        blend_mode = BlendMode$Lighten,
      ) |>
      as_recordedplot()
  )
})
