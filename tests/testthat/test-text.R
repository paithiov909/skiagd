skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- magick::image_graph(width = 720, height = 576)
on.exit(dev.off())

test_that("add_text and add_textpath works", {
  skip_if_not(
    "Noto Sans Mono" %in% list_font_families()[["family"]]
  )
  white <- list(color = "white")
  fontface <- list(family = "Noto Sans Mono", fontsize = 48)
  line_path <- path_transform("M 720 576 L 0 0", c(1, 0, -24, 0, 1, -24, 0, 0, 1))

  vdiffr::expect_doppelganger(
    "text",
    canvas("limegreen") |>
      add_path(line_path, props = paint(!!!white, style = Style$Stroke, width = 4)) |>
      add_text("Hello, Skiagd!", props = paint(!!!white, !!!fontface)) |>
      add_textpath(
        "This is text path text.",
        line_path,
        props = paint(!!!white, !!!fontface)
      ) |>
      as_recordedplot()
  )
})
