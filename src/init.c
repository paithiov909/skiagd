
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP savvy_sk_absolute_fill__impl(SEXP c_arg__size, SEXP c_arg__fill) {
    SEXP res = savvy_sk_absolute_fill__ffi(c_arg__size, c_arg__fill);
    return handle_result(res);
}

SEXP savvy_sk_as_png__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat) {
    SEXP res = savvy_sk_as_png__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat);
    return handle_result(res);
}

SEXP savvy_sk_draw_circle__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__radius) {
    SEXP res = savvy_sk_draw_circle__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__x, c_arg__y, c_arg__radius);
    return handle_result(res);
}

SEXP savvy_sk_draw_diff_rect__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__outer_left, SEXP c_arg__outer_top, SEXP c_arg__outer_right, SEXP c_arg__outer_bottom, SEXP c_arg__outer_rx, SEXP c_arg__outer_ry, SEXP c_arg__inner_left, SEXP c_arg__inner_top, SEXP c_arg__inner_right, SEXP c_arg__inner_bottom, SEXP c_arg__inner_rx, SEXP c_arg__inner_ry) {
    SEXP res = savvy_sk_draw_diff_rect__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__outer_left, c_arg__outer_top, c_arg__outer_right, c_arg__outer_bottom, c_arg__outer_rx, c_arg__outer_ry, c_arg__inner_left, c_arg__inner_top, c_arg__inner_right, c_arg__inner_bottom, c_arg__inner_rx, c_arg__inner_ry);
    return handle_result(res);
}

SEXP savvy_sk_draw_line__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__from_x, SEXP c_arg__from_y, SEXP c_arg__to_x, SEXP c_arg__to_y) {
    SEXP res = savvy_sk_draw_line__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__from_x, c_arg__from_y, c_arg__to_x, c_arg__to_y);
    return handle_result(res);
}

SEXP savvy_sk_draw_path__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat1, SEXP c_arg__props, SEXP c_arg__svg, SEXP c_arg__mat2, SEXP c_arg__fill_type) {
    SEXP res = savvy_sk_draw_path__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat1, c_arg__props, c_arg__svg, c_arg__mat2, c_arg__fill_type);
    return handle_result(res);
}

SEXP savvy_sk_draw_png__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__png_bytes, SEXP c_arg__left_top) {
    SEXP res = savvy_sk_draw_png__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__png_bytes, c_arg__left_top);
    return handle_result(res);
}

SEXP savvy_sk_draw_points__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__mode) {
    SEXP res = savvy_sk_draw_points__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__x, c_arg__y, c_arg__mode);
    return handle_result(res);
}

SEXP savvy_sk_draw_rounded_rect__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__left, SEXP c_arg__top, SEXP c_arg__right, SEXP c_arg__bottom, SEXP c_arg__rx, SEXP c_arg__ry) {
    SEXP res = savvy_sk_draw_rounded_rect__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__left, c_arg__top, c_arg__right, c_arg__bottom, c_arg__rx, c_arg__ry);
    return handle_result(res);
}

SEXP savvy_sk_draw_text__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__text) {
    SEXP res = savvy_sk_draw_text__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__text);
    return handle_result(res);
}

SEXP savvy_sk_draw_textblob__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__text, SEXP c_arg__x, SEXP c_arg__y) {
    SEXP res = savvy_sk_draw_textblob__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__text, c_arg__x, c_arg__y);
    return handle_result(res);
}

SEXP savvy_sk_draw_textpath__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat1, SEXP c_arg__props, SEXP c_arg__text, SEXP c_arg__svg, SEXP c_arg__mat2) {
    SEXP res = savvy_sk_draw_textpath__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat1, c_arg__props, c_arg__text, c_arg__svg, c_arg__mat2);
    return handle_result(res);
}

SEXP savvy_sk_get_text_width__impl(SEXP c_arg__text, SEXP c_arg__props) {
    SEXP res = savvy_sk_get_text_width__ffi(c_arg__text, c_arg__props);
    return handle_result(res);
}

SEXP savvy_sk_list_families__impl(void) {
    SEXP res = savvy_sk_list_families__ffi();
    return handle_result(res);
}

SEXP savvy_sk_matrix_map_point__impl(SEXP c_arg__src_x, SEXP c_arg__src_y, SEXP c_arg__dst_x, SEXP c_arg__dst_y) {
    SEXP res = savvy_sk_matrix_map_point__ffi(c_arg__src_x, c_arg__src_y, c_arg__dst_x, c_arg__dst_y);
    return handle_result(res);
}

SEXP savvy_sk_path_bounds__impl(SEXP c_arg__svg) {
    SEXP res = savvy_sk_path_bounds__ffi(c_arg__svg);
    return handle_result(res);
}

SEXP savvy_sk_path_interpolate__impl(SEXP c_arg__value, SEXP c_arg__first, SEXP c_arg__second) {
    SEXP res = savvy_sk_path_interpolate__ffi(c_arg__value, c_arg__first, c_arg__second);
    return handle_result(res);
}

SEXP savvy_sk_path_transform__impl(SEXP c_arg__svg, SEXP c_arg__mat) {
    SEXP res = savvy_sk_path_transform__ffi(c_arg__svg, c_arg__mat);
    return handle_result(res);
}






SEXP savvy_PaintAttrs_set_attrs__impl(SEXP c_arg__color, SEXP c_arg__style, SEXP c_arg__join, SEXP c_arg__cap, SEXP c_arg__width, SEXP c_arg__miter, SEXP c_arg__fontsize, SEXP c_arg__family, SEXP c_arg__fontface, SEXP c_arg__blend_mode, SEXP c_arg__path_effect, SEXP c_arg__shader) {
    SEXP res = savvy_PaintAttrs_set_attrs__ffi(c_arg__color, c_arg__style, c_arg__join, c_arg__cap, c_arg__width, c_arg__miter, c_arg__fontsize, c_arg__family, c_arg__fontface, c_arg__blend_mode, c_arg__path_effect, c_arg__shader);
    return handle_result(res);
}

SEXP savvy_PathEffect_corner__impl(SEXP c_arg__radius) {
    SEXP res = savvy_PathEffect_corner__ffi(c_arg__radius);
    return handle_result(res);
}

SEXP savvy_PathEffect_dash__impl(SEXP c_arg__intervals, SEXP c_arg__phase) {
    SEXP res = savvy_PathEffect_dash__ffi(c_arg__intervals, c_arg__phase);
    return handle_result(res);
}

SEXP savvy_PathEffect_discrete__impl(SEXP c_arg__length, SEXP c_arg__deviation, SEXP c_arg__seed) {
    SEXP res = savvy_PathEffect_discrete__ffi(c_arg__length, c_arg__deviation, c_arg__seed);
    return handle_result(res);
}

SEXP savvy_PathEffect_get_label__impl(SEXP self__) {
    SEXP res = savvy_PathEffect_get_label__ffi(self__);
    return handle_result(res);
}

SEXP savvy_PathEffect_line_2d__impl(SEXP c_arg__width, SEXP c_arg__transform) {
    SEXP res = savvy_PathEffect_line_2d__ffi(c_arg__width, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_PathEffect_no_effect__impl(void) {
    SEXP res = savvy_PathEffect_no_effect__ffi();
    return handle_result(res);
}

SEXP savvy_PathEffect_path_1d__impl(SEXP c_arg__path, SEXP c_arg__advance, SEXP c_arg__phase, SEXP c_arg__style) {
    SEXP res = savvy_PathEffect_path_1d__ffi(c_arg__path, c_arg__advance, c_arg__phase, c_arg__style);
    return handle_result(res);
}

SEXP savvy_PathEffect_path_2d__impl(SEXP c_arg__path, SEXP c_arg__transform) {
    SEXP res = savvy_PathEffect_path_2d__ffi(c_arg__path, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_PathEffect_sum__impl(SEXP c_arg__first, SEXP c_arg__second) {
    SEXP res = savvy_PathEffect_sum__ffi(c_arg__first, c_arg__second);
    return handle_result(res);
}

SEXP savvy_PathEffect_trim__impl(SEXP c_arg__start, SEXP c_arg__end) {
    SEXP res = savvy_PathEffect_trim__ffi(c_arg__start, c_arg__end);
    return handle_result(res);
}


SEXP savvy_Shader_blend__impl(SEXP c_arg__mode, SEXP c_arg__dst, SEXP c_arg__src) {
    SEXP res = savvy_Shader_blend__ffi(c_arg__mode, c_arg__dst, c_arg__src);
    return handle_result(res);
}

SEXP savvy_Shader_color__impl(SEXP c_arg__rgba) {
    SEXP res = savvy_Shader_color__ffi(c_arg__rgba);
    return handle_result(res);
}

SEXP savvy_Shader_conical_gradient__impl(SEXP c_arg__start, SEXP c_arg__end, SEXP c_arg__radii, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_conical_gradient__ffi(c_arg__start, c_arg__end, c_arg__radii, c_arg__from, c_arg__to, c_arg__mode, c_arg__flags, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_fractal_noise__impl(SEXP c_arg__freq, SEXP c_arg__octaves, SEXP c_arg__seed, SEXP c_arg__tile_size) {
    SEXP res = savvy_Shader_fractal_noise__ffi(c_arg__freq, c_arg__octaves, c_arg__seed, c_arg__tile_size);
    return handle_result(res);
}

SEXP savvy_Shader_from_picture__impl(SEXP c_arg__img, SEXP c_arg__mode, SEXP c_arg__tile_size, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_from_picture__ffi(c_arg__img, c_arg__mode, c_arg__tile_size, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_from_png__impl(SEXP c_arg__png_bytes, SEXP c_arg__mode, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_from_png__ffi(c_arg__png_bytes, c_arg__mode, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_get_label__impl(SEXP self__) {
    SEXP res = savvy_Shader_get_label__ffi(self__);
    return handle_result(res);
}

SEXP savvy_Shader_linear_gradient__impl(SEXP c_arg__start, SEXP c_arg__end, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_linear_gradient__ffi(c_arg__start, c_arg__end, c_arg__from, c_arg__to, c_arg__mode, c_arg__flags, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_no_shader__impl(void) {
    SEXP res = savvy_Shader_no_shader__ffi();
    return handle_result(res);
}

SEXP savvy_Shader_radial_gradient__impl(SEXP c_arg__center, SEXP c_arg__radius, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_radial_gradient__ffi(c_arg__center, c_arg__radius, c_arg__from, c_arg__to, c_arg__mode, c_arg__flags, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_sweep_gradient__impl(SEXP c_arg__center, SEXP c_arg__start_angle, SEXP c_arg__end_angle, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform) {
    SEXP res = savvy_Shader_sweep_gradient__ffi(c_arg__center, c_arg__start_angle, c_arg__end_angle, c_arg__from, c_arg__to, c_arg__mode, c_arg__flags, c_arg__transform);
    return handle_result(res);
}

SEXP savvy_Shader_turbulence__impl(SEXP c_arg__freq, SEXP c_arg__octaves, SEXP c_arg__seed, SEXP c_arg__tile_size) {
    SEXP res = savvy_Shader_turbulence__ffi(c_arg__freq, c_arg__octaves, c_arg__seed, c_arg__tile_size);
    return handle_result(res);
}




static const R_CallMethodDef CallEntries[] = {
    {"savvy_sk_absolute_fill__impl", (DL_FUNC) &savvy_sk_absolute_fill__impl, 2},
    {"savvy_sk_as_png__impl", (DL_FUNC) &savvy_sk_as_png__impl, 3},
    {"savvy_sk_draw_circle__impl", (DL_FUNC) &savvy_sk_draw_circle__impl, 7},
    {"savvy_sk_draw_diff_rect__impl", (DL_FUNC) &savvy_sk_draw_diff_rect__impl, 16},
    {"savvy_sk_draw_line__impl", (DL_FUNC) &savvy_sk_draw_line__impl, 8},
    {"savvy_sk_draw_path__impl", (DL_FUNC) &savvy_sk_draw_path__impl, 7},
    {"savvy_sk_draw_png__impl", (DL_FUNC) &savvy_sk_draw_png__impl, 6},
    {"savvy_sk_draw_points__impl", (DL_FUNC) &savvy_sk_draw_points__impl, 7},
    {"savvy_sk_draw_rounded_rect__impl", (DL_FUNC) &savvy_sk_draw_rounded_rect__impl, 10},
    {"savvy_sk_draw_text__impl", (DL_FUNC) &savvy_sk_draw_text__impl, 5},
    {"savvy_sk_draw_textblob__impl", (DL_FUNC) &savvy_sk_draw_textblob__impl, 7},
    {"savvy_sk_draw_textpath__impl", (DL_FUNC) &savvy_sk_draw_textpath__impl, 7},
    {"savvy_sk_get_text_width__impl", (DL_FUNC) &savvy_sk_get_text_width__impl, 2},
    {"savvy_sk_list_families__impl", (DL_FUNC) &savvy_sk_list_families__impl, 0},
    {"savvy_sk_matrix_map_point__impl", (DL_FUNC) &savvy_sk_matrix_map_point__impl, 4},
    {"savvy_sk_path_bounds__impl", (DL_FUNC) &savvy_sk_path_bounds__impl, 1},
    {"savvy_sk_path_interpolate__impl", (DL_FUNC) &savvy_sk_path_interpolate__impl, 3},
    {"savvy_sk_path_transform__impl", (DL_FUNC) &savvy_sk_path_transform__impl, 2},





    {"savvy_PaintAttrs_set_attrs__impl", (DL_FUNC) &savvy_PaintAttrs_set_attrs__impl, 12},
    {"savvy_PathEffect_corner__impl", (DL_FUNC) &savvy_PathEffect_corner__impl, 1},
    {"savvy_PathEffect_dash__impl", (DL_FUNC) &savvy_PathEffect_dash__impl, 2},
    {"savvy_PathEffect_discrete__impl", (DL_FUNC) &savvy_PathEffect_discrete__impl, 3},
    {"savvy_PathEffect_get_label__impl", (DL_FUNC) &savvy_PathEffect_get_label__impl, 1},
    {"savvy_PathEffect_line_2d__impl", (DL_FUNC) &savvy_PathEffect_line_2d__impl, 2},
    {"savvy_PathEffect_no_effect__impl", (DL_FUNC) &savvy_PathEffect_no_effect__impl, 0},
    {"savvy_PathEffect_path_1d__impl", (DL_FUNC) &savvy_PathEffect_path_1d__impl, 4},
    {"savvy_PathEffect_path_2d__impl", (DL_FUNC) &savvy_PathEffect_path_2d__impl, 2},
    {"savvy_PathEffect_sum__impl", (DL_FUNC) &savvy_PathEffect_sum__impl, 2},
    {"savvy_PathEffect_trim__impl", (DL_FUNC) &savvy_PathEffect_trim__impl, 2},

    {"savvy_Shader_blend__impl", (DL_FUNC) &savvy_Shader_blend__impl, 3},
    {"savvy_Shader_color__impl", (DL_FUNC) &savvy_Shader_color__impl, 1},
    {"savvy_Shader_conical_gradient__impl", (DL_FUNC) &savvy_Shader_conical_gradient__impl, 8},
    {"savvy_Shader_fractal_noise__impl", (DL_FUNC) &savvy_Shader_fractal_noise__impl, 4},
    {"savvy_Shader_from_picture__impl", (DL_FUNC) &savvy_Shader_from_picture__impl, 4},
    {"savvy_Shader_from_png__impl", (DL_FUNC) &savvy_Shader_from_png__impl, 3},
    {"savvy_Shader_get_label__impl", (DL_FUNC) &savvy_Shader_get_label__impl, 1},
    {"savvy_Shader_linear_gradient__impl", (DL_FUNC) &savvy_Shader_linear_gradient__impl, 7},
    {"savvy_Shader_no_shader__impl", (DL_FUNC) &savvy_Shader_no_shader__impl, 0},
    {"savvy_Shader_radial_gradient__impl", (DL_FUNC) &savvy_Shader_radial_gradient__impl, 7},
    {"savvy_Shader_sweep_gradient__impl", (DL_FUNC) &savvy_Shader_sweep_gradient__impl, 8},
    {"savvy_Shader_turbulence__impl", (DL_FUNC) &savvy_Shader_turbulence__impl, 4},


    {NULL, NULL, 0}
};

void R_init_skiagd(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
