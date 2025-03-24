skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- magick::image_graph(width = 720, height = 576)
on.exit(dev.off())

test_that("Shader$from_picture works", {
  props <- list(color = "skyblue", style = Style$StrokeAndFill)
  pict <-
    canvas("pink") |>
    add_circle(
      matrix(c(48, 48), ncol = 2), 24,
      props = paint(!!!props)
    )
  vdiffr::expect_doppelganger(
    "from_picture",
    canvas("white") |>
      add_rect(
        matrix(c(0, 0, dev_size()[1], dev_size()[2]), ncol = 4),
        props = paint(
          !!!props, # this doesn't matter here, just for testing reusing props.
          shader = Shader$from_picture(
            pict,
            mode = TileMode$Repeat,
            tile_size = c(96, 96),
            transform = c(1, 0, 24, 0, 1, 0, 0, 0, 1)
          )
        )
      ) |>
      as_recordedplot()
  )
})
