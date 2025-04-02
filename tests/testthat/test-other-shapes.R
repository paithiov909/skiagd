skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- magick::image_graph(width = 720, height = 576)
on.exit(dev.off())

test_that("add_atlas works", {
  rect_size <- as.integer(c(24, 11))
  image <- canvas("transparent", canvas_size = rect_size) |>
    add_rounded_rect(
      matrix(c(0, 0, rect_size[1], rect_size[2]), ncol = 4),
      matrix(c(6, 6), ncol = 2),
      props = paint(canvas_size = rect_size, color = "snow", style = Style$Fill)
    ) |>
    as_png(props = paint(canvas_size = rect_size))

  size <- dev_size()
  num_rects <- 250
  tx <- 25 + seq_len(num_rects) * ((size[1] / 2) / num_rects)
  ty <- 50 + (seq_len(num_rects) * ((size[2] / 2) / num_rects))
  r <- atan2(size[2] - ty, size[1] - tx)

  trans <- data.frame(
    scale = rep_len(1, num_rects),
    radians = r,
    tx = size[1] / 4 * cos(tx) + size[1] / 2,
    ty = size[2] / 4 * sin(ty) + size[2] / 2,
    ax = rep_len(rect_size[1] / 2, num_rects),
    ay = rep_len(rect_size[2] / 2, num_rects)
  )

  vdiffr::expect_doppelganger(
    "atlas",
    canvas("violetred") |>
      add_atlas(image, trans) |>
      as_recordedplot()
  )
})
