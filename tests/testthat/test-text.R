skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- grDevices::png(tempfile(fileext = ".png"), width = 720, height = 576)
on.exit(dev.off())

test_that("add_text and add_textpath works", {
  skip_if_not(
    "Noto Sans Mono" %in% list_font_families()[["family"]]
  )
  white <- list(color = "white")
  fontface <- list(family = "Noto Sans Mono", fontsize = 48)

  texts <- c(
    "Hello, Skiagd!",
    "This is a curved text."
  )
  rsx_trans_1 <-
    text_info(texts[1], props = paint(!!!fontface)) |>
    dplyr::reframe(
      sc = rep_len(1, n_chars),
      rot = rep_len(0, n_chars),
      x = width / n_chars * seq_len(n_chars),
      y = rep_len(48, n_chars),
      ax = rep_len(0, n_chars),
      ay = rep_len(0, n_chars),
      .by = id
    )
  rsx_trans_2 <-
    text_info(texts[2], props = paint(!!!fontface)) |>
    dplyr::reframe(
      sc = rep_len(1, n_chars),
      rot = seq(-pi / 4, pi / 4, length.out = n_chars),
      x = width / n_chars * seq_len(n_chars),
      y = (seq(-pi, 0, length.out = n_chars) |> sin()) * width / 4 + 288,
      ax = rep_len(0, n_chars),
      ay = rep_len(0, n_chars),
      .by = id
    )

  vdiffr::expect_doppelganger(
    "text",
    canvas("limegreen") |>
      add_text(
        texts,
        rsx_trans = rbind(rsx_trans_1, rsx_trans_2) |>
          dplyr::select(!"id") |>
          as.matrix(),
        props = paint(!!!white, !!!fontface)
      ) |>
      as_recordedplot()
  )
})
