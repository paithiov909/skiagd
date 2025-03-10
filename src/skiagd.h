#pragma once

#ifndef R_NO_REMAP
#define R_NO_REMAP
#endif

#include <vector>
#include <cpp11.hpp>
#include <R_ext/GraphicsEngine.h>  // for R_RGBA
#include "rust/target/cxxbridge/skiagd/src/lib.rs.h"

#define BEGIN_RUST try {
#define END_RUST                  \
  }                               \
  catch (const std::bad_alloc&) { \
    cpp11::stop("out of memory"); \
  }                               \
  catch (std::exception & e) {    \
    cpp11::stop(e.what());        \
  }

typedef std::vector<std::uint8_t> Img;
typedef rust::cxxbridge1::Vec<std::uint8_t> BytesBuf;
typedef rust::cxxbridge1::Box<Canvas> SkiaCanvas;

inline Img raws_to_img(const cpp11::raws& buf) {
  return Img(buf.begin(), buf.end());
}

inline cpp11::raws bytes_to_raws(const BytesBuf& buf) {
  return cpp11::writable::raws(buf.begin(), buf.end());
}

inline uint32_t vec2color(const cpp11::integers& vec) {
  if (vec.size() != 4) {
    cpp11::stop("invalid color");
  }
  return R_RGBA(vec[0], vec[1], vec[2], vec[3]);
}

// col: u32,
// fill: u32,
// ljoin: u32,
// lend: u32,
// lty: u32,
// lwd: f32,
// lmiter: f32,
// blend_mode: u32,
struct PaintProps {
 public:
  PaintProps(uint32_t col = 0, uint32_t fill = 0, uint32_t ljoin = 0,
             uint32_t lend = 0, uint32_t lty = 0, float lwd = 0,
             float lmiter = 0, uint32_t blend_mode = 0)
      : col(col),
        fill(fill),
        ljoin(ljoin),
        lend(lend),
        lty(lty),
        lwd(lwd),
        lmiter(lmiter),
        blend_mode(blend_mode) {}
  void apply(SkiaCanvas& canvas) {
    canvas->set_paint_props(col, fill, ljoin, lend, lty, lwd, lmiter,
                            blend_mode);
  }
  uint32_t col;
  uint32_t fill;
  uint32_t ljoin;
  uint32_t lend;
  uint32_t lty;
  float lwd;
  float lmiter;
  uint32_t blend_mode;
};

inline PaintProps parse_props(const cpp11::list& props) {
  PaintProps p{vec2color(props["col"]),
               cpp11::as_cpp<uint32_t>(props["fill"]),
               cpp11::as_cpp<uint32_t>(props["ljoin"]),
               cpp11::as_cpp<uint32_t>(props["lend"]),
               cpp11::as_cpp<uint32_t>(props["lty"]),
               cpp11::as_cpp<float>(props["lwd"]),
               cpp11::as_cpp<float>(props["lmiter"]),
               cpp11::as_cpp<uint32_t>(props["blend_mode"])};
  return p;
}
