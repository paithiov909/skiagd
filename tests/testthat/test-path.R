# to prevent opening default graphics device
dev <- magick::image_graph(width = 720, height = 576)
on.exit(dev.off())

test_that("add_rect works", {
  vdiffr::expect_doppelganger(
    "rect",
    canvas("red") |>
      add_rect(
        matrix(c(120, 40, 440, 120), ncol = 4),
        props = paint(color = "snow", width = 16, style = Style$Stroke, join = Join$Round)
      ) |>
      add_rect(
        matrix(c(160, 220, 480, 300), ncol = 4),
        props = paint(color = "snow", width = 16, style = Style$Stroke, join = Join$Bevel)
      ) |>
      add_rect(
        matrix(c(200, 400, 520, 480), ncol = 4),
        props = paint(color = "snow", width = 16, style = Style$Stroke, join = Join$Miter)
      ) |>
      as_recordedplot()
  )
})

test_that("add_line works", {
  vdiffr::expect_doppelganger(
    "line",
    canvas("white") |>
      add_line(
        matrix(c(20, 20), ncol = 2), matrix(c(320, 320), ncol = 2),
        props = paint(color = "steelblue", width = 16, cap = Cap$Round)
      ) |>
      add_line(
        matrix(c(20, 20), ncol = 2), matrix(c(320, 320), ncol = 2),
        props = paint(color = "purple", width = 16, cap = Cap$Butt, transform = c(1, 0, 120, 0, 1, 0, 0, 0, 1))
      ) |>
      add_line(
        matrix(c(20, 20), ncol = 2), matrix(c(320, 320), ncol = 2),
        props = paint(color = "blue", width = 16, cap = Cap$Square, transform = c(1, 0, 120, 0, 1, 0, 0, 0, 1))
      ) |>
      as_recordedplot()
  )
})

test_that("add_path works", {
  star <- "M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 155 L 0 93 L 88 80 L 128 0 Z"
  vdiffr::expect_doppelganger(
    "path",
    canvas("limegreen") |>
      add_path(
        star,
        transform = c(1, 0, 256, 0, 1, 128, 0, 0, 1),
        props = paint(color = "yellow")
      ) |>
      add_path(
        star,
        transform = c(0.707, -0.707, 256, 0.707, 0.707, 128, 0, 0, 1),
        props = paint(color = "yellow", blend_mode = BlendMode$Exclusion)
      ) |>
      as_recordedplot()
  )
})
