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
