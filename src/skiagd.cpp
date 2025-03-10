#include "skiagd.h"

[[cpp11::register]]
bool test(const cpp11::list& props) {
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(300, 300);
  canvas->new_page(R_RGBA(0, 255, 0, 255));
  p.apply(canvas);
  canvas->translate(20, 30);
  canvas->svg_path(
      "M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 "
      "155 L 0 93 L 88 80 L 128 0 Z");
  canvas->line(0, 0, 300, 300);
  canvas->save_png("test.png");
  END_RUST
  return true;
}

[[cpp11::register]]
std::string sk_save_png(const cpp11::integers& size,
                        const cpp11::raws& curr_bytes,
                        const std::string& filename) {
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);  // without new_page
  canvas->save_png(filename);
  END_RUST
  cpp11::message("Saved to %s", filename.c_str());
  return filename;
}

[[cpp11::register]]
cpp11::raws sk_absolute_fill(const cpp11::integers& size,
                             const cpp11::integers& fill) {
  BytesBuf bytes;
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  canvas->new_page(vec2color(fill));
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}

[[cpp11::register]]
cpp11::raws sk_points(const cpp11::integers& size,
                      const cpp11::raws& curr_bytes,
                      const cpp11::doubles_matrix<>& mat_pts,
                      const cpp11::list& props, const cpp11::integers& mode) {
  if (mat_pts.ncol() < 2) {
    cpp11::stop("point must have at least 2 columns");
  }
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  p.apply(canvas);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  for (R_xlen_t i = 0; i < mat_pts.nrow(); i++) {
    canvas->points(mat_pts(i, 0), mat_pts(i, 1), cpp11::as_cpp<uint32_t>(mode));
  }
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}

[[cpp11::register]]
cpp11::raws sk_line(const cpp11::integers& size, const cpp11::raws& curr_bytes,
                    const cpp11::doubles_matrix<>& mat_from,
                    const cpp11::doubles_matrix<>& mat_to,
                    const cpp11::list& props) {
  if (mat_from.nrow() != mat_to.nrow()) {
    cpp11::stop("x and y must have the same length");
  }
  if (mat_from.ncol() < 2 || mat_to.ncol() < 2) {
    cpp11::stop("x and y must have at least 2 columns");
  }
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  p.apply(canvas);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  for (R_xlen_t i = 0; i < mat_from.nrow(); i++) {
    canvas->line(mat_from(i, 0), mat_from(i, 1), mat_to(i, 0), mat_to(i, 1));
  }
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}

[[cpp11::register]]
cpp11::raws sk_circle(const cpp11::integers& size,
                      const cpp11::raws& curr_bytes,
                      const cpp11::doubles_matrix<>& mat_center,
                      const cpp11::doubles& vec_radius,
                      const cpp11::list& props) {
  if (mat_center.nrow() != vec_radius.size()) {
    cpp11::stop("center and radius must have the same length");
  }
  if (mat_center.ncol() < 2) {
    cpp11::stop("center must have at least 2 columns");
  }
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  p.apply(canvas);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  for (R_xlen_t i = 0; i < mat_center.nrow(); i++) {
    canvas->circle(mat_center(i, 0), mat_center(i, 1), vec_radius[i]);
  }
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}

[[cpp11::register]]
cpp11::raws sk_irect(const cpp11::integers& size, const cpp11::raws& curr_bytes,
                     const cpp11::integers_matrix<>& mat_rect,
                     const cpp11::list& props) {
  if (mat_rect.ncol() < 4) {
    cpp11::stop("rect must have at least 4 columns");
  }
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  p.apply(canvas);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  for (R_xlen_t i = 0; i < mat_rect.nrow(); i++) {
    canvas->irect(mat_rect(i, 0), mat_rect(i, 1), mat_rect(i, 2),
                  mat_rect(i, 3));
  }
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}

[[cpp11::register]]
cpp11::raws sk_svg_path(const cpp11::integers& size,
                        const cpp11::raws& curr_bytes,
                        const cpp11::strings& vec_svg,
                        const cpp11::integers& translate,
                        const cpp11::integers& scale,
                        const cpp11::list& props) {
  if (translate.size() != 2 || scale.size() != 2) {
    cpp11::stop("translate and scale must have length 2");
  }
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  p.apply(canvas);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  canvas->translate(translate[0], translate[1]);
  canvas->scale(scale[0], scale[1]);
  for (const std::string& svg_path : vec_svg) {
    canvas->svg_path(svg_path);
  }
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}
