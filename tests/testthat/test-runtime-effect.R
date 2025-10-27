# to prevent opening default graphics device
dev <- grDevices::png(tempfile(fileext = ".png"), width = 720, height = 576)
on.exit(dev.off())

test_that("ImageFilter$runtime_shader throws an error when uniforms are invalid or mismatched", {
  effect <-
    RuntimeEffect$make(
      R"{
      uniform shader image;
      uniform vec2 resolution;
      uniform float4x4 mat;
      vec4 main(vec2 fragCoord) {
        vec2 uv = fragCoord / resolution;
        return vec4(uv.x, uv.y, .6, 1.0);
      }
    }"
    )
  # from `make_builder`
  expect_error(
    ImageFilter$runtime_shader(effect, list(foo = "bar")),
  )
  # from `runtime_shader`
  expect_error(
    ImageFilter$runtime_shader(effect, list(resolution = 1:8)),
  )
  expect_error(
    ImageFilter$runtime_shader(
      effect,
      list(resolution = as.double(dev_size()), mat = as.double(1:12))
    ),
  )
  # this should be compiled successfully
  expect_no_error(
    ImageFilter$runtime_shader(
      effect,
      list(resolution = as.double(dev_size()), mat = as.double(1:16))
    ),
  )
})

test_that("ImageFilter$runtime_shader works", {
  skip_on_cran()
  skip_on_ci()

  canvas_size <- dev_size()

  effect <- RuntimeEffect$make(
    R"{
    uniform shader image;
    uniform vec2 resolution;

 	  vec4 main(vec2 fragCoord) {
      vec2 uv = fragCoord / resolution;
   		return distance(uv, vec2(.5)) > .2 ? image.eval(fragCoord).gbra : image.eval(fragCoord).rgba;
  	}
  }"
  )

  imgf <-
    ImageFilter$runtime_shader(
      effect,
      list(resolution = as.double(canvas_size))
    )

  vdiffr::expect_doppelganger(
    "runtime_shader-1",
    canvas("white") |>
      add_rect(
        matrix(c(0, 0, canvas_size), ncol = 4),
        props = paint(
          color = "steelblue",
          image_filter = imgf,
        )
      ) |>
      as_recordedplot()
  )

  effect <- RuntimeEffect$make(
    R"{
    uniform shader image;
    uniform vec2 resolution;

 	  vec4 main(vec2 fragCoord) {
      vec2 uv = fragCoord / resolution;
   		return uv.y < 0.5 ? image.eval(fragCoord).gbra : image.eval(fragCoord).rgba;
  	}
  }"
  )

  imgf <-
    ImageFilter$runtime_shader(
      effect,
      list(resolution = as.double(canvas_size))
    )

  vdiffr::expect_doppelganger(
    "runtime_shader-2",
    canvas("white") |>
      add_rect(
        matrix(c(0, 0, canvas_size), ncol = 4),
        props = paint(
          color = "steelblue",
          image_filter = imgf,
        )
      ) |>
      as_recordedplot()
  )
})

test_that("Shader$from_runtime_effect works", {
  skip_on_cran()
  skip_on_ci()

  canvas_size <- dev_size()

  effect <-
    RuntimeEffect$make(
      R"{
      uniform vec2 resolution;
      vec4 main(vec2 pos) {
        vec2 uv = pos/resolution;
        return vec4(uv.x, uv.y, 0.5, 1);
      }
    }"
    )

  shdr <-
    Shader$from_runtime_effect(
      effect,
      list(resolution = as.double(canvas_size))
    )

  vdiffr::expect_doppelganger(
    "from_runtime_effect",
    canvas("white") |>
      add_rect(
        matrix(c(0, 0, canvas_size), ncol = 4),
        props = paint(
          color = "white",
          shader = shdr,
        )
      ) |>
      as_recordedplot()
  )
})
