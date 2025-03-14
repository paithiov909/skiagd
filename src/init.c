
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

SEXP savvy_sk_draw_irect__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__left, SEXP c_arg__top, SEXP c_arg__right, SEXP c_arg__bottom) {
    SEXP res = savvy_sk_draw_irect__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__left, c_arg__top, c_arg__right, c_arg__bottom);
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

SEXP savvy_sk_matrix_default__impl(void) {
    SEXP res = savvy_sk_matrix_default__ffi();
    return handle_result(res);
}





SEXP savvy_PaintAttrs_set_attrs__impl(SEXP c_arg__color, SEXP c_arg__style, SEXP c_arg__join, SEXP c_arg__cap, SEXP c_arg__width, SEXP c_arg__miter, SEXP c_arg__blend_mode, SEXP c_arg__path_effect) {
    SEXP res = savvy_PaintAttrs_set_attrs__ffi(c_arg__color, c_arg__style, c_arg__join, c_arg__cap, c_arg__width, c_arg__miter, c_arg__blend_mode, c_arg__path_effect);
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

SEXP savvy_PathEffect_line_2d__impl(SEXP c_arg__width, SEXP c_arg__mat) {
    SEXP res = savvy_PathEffect_line_2d__ffi(c_arg__width, c_arg__mat);
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

SEXP savvy_PathEffect_path_2d__impl(SEXP c_arg__path, SEXP c_arg__mat) {
    SEXP res = savvy_PathEffect_path_2d__ffi(c_arg__path, c_arg__mat);
    return handle_result(res);
}

SEXP savvy_PathEffect_trim__impl(SEXP c_arg__start, SEXP c_arg__end) {
    SEXP res = savvy_PathEffect_trim__ffi(c_arg__start, c_arg__end);
    return handle_result(res);
}




static const R_CallMethodDef CallEntries[] = {
    {"savvy_sk_absolute_fill__impl", (DL_FUNC) &savvy_sk_absolute_fill__impl, 2},
    {"savvy_sk_as_png__impl", (DL_FUNC) &savvy_sk_as_png__impl, 3},
    {"savvy_sk_draw_circle__impl", (DL_FUNC) &savvy_sk_draw_circle__impl, 7},
    {"savvy_sk_draw_irect__impl", (DL_FUNC) &savvy_sk_draw_irect__impl, 8},
    {"savvy_sk_draw_line__impl", (DL_FUNC) &savvy_sk_draw_line__impl, 8},
    {"savvy_sk_draw_path__impl", (DL_FUNC) &savvy_sk_draw_path__impl, 7},
    {"savvy_sk_draw_png__impl", (DL_FUNC) &savvy_sk_draw_png__impl, 6},
    {"savvy_sk_draw_points__impl", (DL_FUNC) &savvy_sk_draw_points__impl, 7},
    {"savvy_sk_matrix_default__impl", (DL_FUNC) &savvy_sk_matrix_default__impl, 0},




    {"savvy_PaintAttrs_set_attrs__impl", (DL_FUNC) &savvy_PaintAttrs_set_attrs__impl, 8},
    {"savvy_PathEffect_corner__impl", (DL_FUNC) &savvy_PathEffect_corner__impl, 1},
    {"savvy_PathEffect_dash__impl", (DL_FUNC) &savvy_PathEffect_dash__impl, 2},
    {"savvy_PathEffect_discrete__impl", (DL_FUNC) &savvy_PathEffect_discrete__impl, 3},
    {"savvy_PathEffect_get_label__impl", (DL_FUNC) &savvy_PathEffect_get_label__impl, 1},
    {"savvy_PathEffect_line_2d__impl", (DL_FUNC) &savvy_PathEffect_line_2d__impl, 2},
    {"savvy_PathEffect_no_effect__impl", (DL_FUNC) &savvy_PathEffect_no_effect__impl, 0},
    {"savvy_PathEffect_path_1d__impl", (DL_FUNC) &savvy_PathEffect_path_1d__impl, 4},
    {"savvy_PathEffect_path_2d__impl", (DL_FUNC) &savvy_PathEffect_path_2d__impl, 2},
    {"savvy_PathEffect_trim__impl", (DL_FUNC) &savvy_PathEffect_trim__impl, 2},


    {NULL, NULL, 0}
};

void R_init_skiagd(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
