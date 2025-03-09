#include "skiagd.h"

[[cpp11::register]]
bool test1() {
  auto canvas = skia_canvas(300, 300);
  canvas->new_page(R_RGBA(0, 255, 0, 255));
  canvas->save_png("test.png");
  return true;
}

[[cpp11::register]]
uint32_t test2(const cpp11::integers& fill) {
  return vec2color(fill);
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
cpp11::raws sk_circle(const cpp11::integers& size,
                      const cpp11::raws& curr_bytes, double x, double y,
                      double r, const cpp11::list& props) {
  BytesBuf bytes;
  PaintProps p = parse_props(props);
  BEGIN_RUST
  auto canvas = skia_canvas(size[0], size[1]);
  canvas->set_paint_props(p.col, p.fill, p.ljoin, p.lend, p.lty, p.lwd,
                          p.lmiter, p.blend_mode);
  canvas->read_bytes(raws_to_img(curr_bytes), 0, 0);
  canvas->circle(x, y, r);
  bytes = canvas->save_bytes();
  END_RUST
  return bytes_to_raws(bytes);
}
