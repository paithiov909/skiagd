# to prevent opening default graphics device
dev <- grDevices::png(tempfile(), width = 720, height = 576)
on.exit(dev.off(), add = TRUE)

test_that("paint rejects transform", {
  expect_error(
    paint(transform = diag(3)),
    "Transforming the previous picture has been removed"
  )
})

test_that("picture can be replayed after many add calls", {
  size <- c(320L, 240L)
  props <- paint(
    canvas_size = size,
    color = "snow",
    style = Style$Fill
  )

  img <- canvas("navy", canvas_size = size) |>
    purrr::reduce(seq_len(180), \(cv, i) {
      x <- 20 + ((i * 11) %% 280)
      y <- 20 + ((i * 7) %% 200)

      cv |>
        add_circle(
          matrix(c(x, y), ncol = 2),
          radius = 6 + (i %% 10),
          props = props
        )
    }, .init = _)

  png <- as_png(img, props = paint(canvas_size = size))
  expect_type(png, "raw")
  expect_gt(length(png), 0)
})

test_that("freeze still returns a reusable picture", {
  size <- c(160L, 120L)
  img <- canvas("white", canvas_size = size) |>
    add_circle(
      matrix(c(80, 60), ncol = 2),
      radius = 24,
      props = paint(canvas_size = size, color = "tomato", style = Style$Fill)
    )

  frozen <- freeze(img, props = paint(canvas_size = size))
  png <- as_png(frozen, props = paint(canvas_size = size))

  expect_type(frozen, "raw")
  expect_type(png, "raw")
  expect_gt(length(frozen), 0)
  expect_gt(length(png), 0)
})
