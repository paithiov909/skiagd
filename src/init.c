
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

SEXP savvy_sk_draw_path__impl(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__svg) {
    SEXP res = savvy_sk_draw_path__ffi(c_arg__size, c_arg__curr_bytes, c_arg__mat, c_arg__props, c_arg__svg);
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




SEXP savvy_PaintProps_set_props__impl(SEXP c_arg__color, SEXP c_arg__style, SEXP c_arg__join, SEXP c_arg__cap, SEXP c_arg__lty, SEXP c_arg__width, SEXP c_arg__miter, SEXP c_arg__blend_mode) {
    SEXP res = savvy_PaintProps_set_props__ffi(c_arg__color, c_arg__style, c_arg__join, c_arg__cap, c_arg__lty, c_arg__width, c_arg__miter, c_arg__blend_mode);
    return handle_result(res);
}




static const R_CallMethodDef CallEntries[] = {
    {"savvy_sk_absolute_fill__impl", (DL_FUNC) &savvy_sk_absolute_fill__impl, 2},
    {"savvy_sk_as_png__impl", (DL_FUNC) &savvy_sk_as_png__impl, 3},
    {"savvy_sk_draw_circle__impl", (DL_FUNC) &savvy_sk_draw_circle__impl, 7},
    {"savvy_sk_draw_irect__impl", (DL_FUNC) &savvy_sk_draw_irect__impl, 8},
    {"savvy_sk_draw_line__impl", (DL_FUNC) &savvy_sk_draw_line__impl, 8},
    {"savvy_sk_draw_path__impl", (DL_FUNC) &savvy_sk_draw_path__impl, 5},
    {"savvy_sk_draw_png__impl", (DL_FUNC) &savvy_sk_draw_png__impl, 6},
    {"savvy_sk_draw_points__impl", (DL_FUNC) &savvy_sk_draw_points__impl, 7},
    {"savvy_sk_matrix_default__impl", (DL_FUNC) &savvy_sk_matrix_default__impl, 0},



    {"savvy_PaintProps_set_props__impl", (DL_FUNC) &savvy_PaintProps_set_props__impl, 8},


    {NULL, NULL, 0}
};

void R_init_skiagd(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
