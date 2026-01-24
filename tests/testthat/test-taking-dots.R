skip_on_cran()
skip_on_ci()

# to prevent opening default graphics device
dev <- grDevices::png(nullfile(), width = 720, height = 576)
on.exit(dev.off())

circle <- function(amp, freq, phase) {
  amp * 1i^(freq * seq(0, 600, length.out = 260) + phase)
}

test_that("drawing functions can take width and color", {
  a <- 0.1234 * 5
  l <- sin(pi * 2 * a - .5) + 1
  z <- circle(1, 1, 0) +
    circle(l, ceiling(a), -8 * a) +
    circle(l / 2 - 1, ceiling(((-a + 2.5) %% 5) - 5), -4 * a)
  z2 <- c(z[-1], z[1])

  hue <- (a + (Re(z / 10))) %% 1
  colors <- grDevices::hsv(hue, 0.65, 1, alpha = 1)

  size <- dev_size() / 2
  trans <- matrix(c(100, 0, size[1], 0, 100, size[2], 0, 0, 1), ncol = 3)

  vdiffr::expect_doppelganger(
    "mystery-rose",
    canvas("#04010F") |>
      add_circle(
        matrix(c(Re(z), Im(z), rep_len(1, length(z))), ncol = 3) %*% trans,
        rep_len(2, length(z)),
        color = col2rgba(colors)
      ) |>
      add_line(
        matrix(c(Re(z), Im(z), rep_len(1, length(z))), ncol = 3) %*% trans,
        matrix(c(Re(z2), Im(z2), rep_len(1, length(z))), ncol = 3) %*% trans,
        width = rep(0.1 + seq(0, 1, length.out = length(z) / 10), each = 10),
        color = col2rgba(colors)
      ) |>
      as_recordedplot()
  )
})
