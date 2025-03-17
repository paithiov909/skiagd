skip_on_ci()

# to prevent opening default graphics device
dev <- magick::image_graph(width = 720, height = 576)
on.exit(dev.off())

test_that("add_cicle works", {
  vdiffr::expect_doppelganger(
    "circle",
    canvas("blue") |>
      add_circle(
        matrix(c(240, 288), ncol = 2), 100,
        props = paint(color = "snow", style = Style$StrokeAndFill)
      ) |>
      add_circle(
        matrix(c(480, 288), ncol = 2), 100,
        props = paint(color = "snow", width = 6, style = Style$Stroke)
      ) |>
      as_recordedplot()
  )
})

test_that("add_cicle with PathEffect works", {
  vdiffr::expect_doppelganger(
    "path_effect",
    canvas("navy") |>
      add_circle(
        matrix(c(120, 120), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke,
          path_effect = PathEffect$discrete(10, 4, 0)
        )
      ) |>
      add_circle(
        matrix(c(120, 288), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke,
          path_effect = PathEffect$dash(c(10, 10), 0)
        )
      ) |>
      add_circle(
        matrix(c(120, 456), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke,
          path_effect = PathEffect$path_1d(
            "M -10 0 L 0 -10, 10 0, 0 10 Z",
            20, 0, "rotate"
          )
        )
      ) |>
      add_circle(
        matrix(c(480, 120), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke,
          path_effect = PathEffect$path_2d(
            "M -10 0 L 0 -10, 10 0, 0 10 Z",
            c(24, 0, 0, 0, 24, 0, 0, 0, 1)
          )
        )
      ) |>
      add_circle(
        matrix(c(480, 288), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke,
          path_effect = PathEffect$line_2d(6, c(16, 0, 0, 0, 16, 0, 0, 0, 1))
        )
      ) |>
      add_circle(
        matrix(c(480, 456), ncol = 2), 100,
        props = paint(
          color = "snow", width = 6, style = Style$Stroke
        )
      ) |>
      as_recordedplot()
  )
})
