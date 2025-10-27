skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- grDevices::png(tempfile(fileext = ".png"), width = 720, height = 576)
on.exit(dev.off())

test_that("add_rect works", {
  vdiffr::expect_doppelganger(
    "rect",
    canvas("red") |>
      add_rect(
        matrix(c(120, 40, 440, 120), ncol = 4),
        props = paint(
          color = "snow",
          width = 16,
          style = Style$Stroke,
          join = Join$Round
        )
      ) |>
      add_rect(
        matrix(c(160, 220, 480, 300), ncol = 4),
        props = paint(
          color = "snow",
          width = 16,
          style = Style$Stroke,
          join = Join$Bevel
        )
      ) |>
      add_rect(
        matrix(c(200, 400, 520, 480), ncol = 4),
        props = paint(
          color = "snow",
          width = 16,
          style = Style$Stroke,
          join = Join$Miter
        )
      ) |>
      as_recordedplot()
  )
  vdiffr::expect_doppelganger(
    "rounded_rect",
    canvas("tomato") |>
      add_rect(
        matrix(c(120, 40, 440, 480), ncol = 4),
        radii = matrix(c(160, 160), ncol = 2),
        props = paint(color = "snow", width = 16, style = Style$Stroke)
      ) |>
      add_rect(
        matrix(
          c(160, 200, 480, 300, 240, 80, 360, 440),
          nrow = 2,
          byrow = TRUE
        ),
        radii = matrix(c(24, 48, 24, 48), ncol = 2),
        props = paint(color = "yellow", width = 16, style = Style$Stroke)
      ) |>
      as_recordedplot()
  )
})

test_that("add_diff_rect works", {
  vdiffr::expect_doppelganger(
    "diff_rect",
    canvas("coral") |>
      add_diff_rect(
        matrix(c(120, 40, 440, 480), ncol = 4),
        matrix(c(240, 80, 360, 440), ncol = 4),
        outer_radii = matrix(c(48, 48), ncol = 2),
        inner_radii = matrix(c(16, 16), ncol = 2),
        props = paint(color = "snow", width = 16, style = Style$Fill)
      ) |>
      as_recordedplot()
  )
})

test_that("add_line works", {
  vdiffr::expect_doppelganger(
    "line",
    canvas("white") |>
      add_line(
        matrix(c(20, 20), ncol = 2),
        matrix(c(320, 320), ncol = 2),
        props = paint(color = "steelblue", width = 16, cap = Cap$Round)
      ) |>
      add_line(
        matrix(c(20, 20), ncol = 2),
        matrix(c(320, 320), ncol = 2),
        props = paint(
          color = "purple",
          width = 16,
          cap = Cap$Butt,
          transform = c(1, 0, 120, 0, 1, 0, 0, 0, 1)
        )
      ) |>
      add_line(
        matrix(c(20, 20), ncol = 2),
        matrix(c(320, 320), ncol = 2),
        props = paint(
          color = "blue",
          width = 16,
          cap = Cap$Square,
          transform = c(1, 0, 120, 0, 1, 0, 0, 0, 1)
        )
      ) |>
      as_recordedplot()
  )
})

test_that("add_point works", {
  size <- dev_size()
  deg2rad <- function(deg) deg * (pi / 180)

  i <- seq_len(360)
  r <- 150 * abs(sin(deg2rad(4 * i)))
  mat <-
    data.frame(
      x = r * cos(deg2rad(360 * i / 360)) + size[1] / 2,
      y = r * sin(deg2rad(360 * i / 360)) + size[2] / 2
    ) |>
    as.matrix()

  vdiffr::expect_doppelganger(
    "point_as_points",
    canvas("snow") |>
      add_point(
        mat,
        props = paint(color = "red", width = 8, point_mode = PointMode$Points)
      ) |>
      as_recordedplot()
  )

  vdiffr::expect_doppelganger(
    "point_as_lines",
    canvas("snow") |>
      add_point(
        mat,
        group = rep(1:20, each = 18),
        color = grDevices::hsv(
          seq(0, 1, length.out = 20),
          0.65,
          1,
          alpha = 1
        ) |>
          grDevices::col2rgb(alpha = TRUE),
        props = paint(color = "red", width = 8, point_mode = PointMode$Lines)
      ) |>
      as_recordedplot()
  )

  vdiffr::expect_doppelganger(
    "point_as_polygon",
    canvas("snow") |>
      add_point(
        mat,
        props = paint(color = "red", width = 8, point_mode = PointMode$Polygon)
      ) |>
      as_recordedplot()
  )
})

test_that("add_path with PathEffect$trim work", {
  star <- "M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 155 L 0 93 L 88 80 L 128 0 Z"
  vdiffr::expect_doppelganger(
    "path",
    canvas("limegreen") |>
      add_path(
        star,
        rsx_trans = matrix(c(1, 0, 256, 128, 0, 0), ncol = 6),
        props = paint(color = "yellow")
      ) |>
      add_path(
        star,
        rsx_trans = matrix(c(1, pi / 4, 256, 128, 0, 0), ncol = 6),
        props = paint(color = "yellow", blend_mode = BlendMode$Exclusion)
      ) |>
      add_path(
        star,
        rsx_trans = matrix(c(1, pi / 4, 256, 128, 0, 0), ncol = 6),
        props = paint(
          color = "black",
          width = 16,
          style = Style$Stroke,
          path_effect = PathEffect$trim(0.2, 0.9)
        )
      ) |>
      as_recordedplot()
  )
})
