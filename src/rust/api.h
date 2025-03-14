SEXP savvy_sk_absolute_fill__ffi(SEXP c_arg__size, SEXP c_arg__fill);
SEXP savvy_sk_as_png__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat);
SEXP savvy_sk_draw_circle__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__radius);
SEXP savvy_sk_draw_irect__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__left, SEXP c_arg__top, SEXP c_arg__right, SEXP c_arg__bottom);
SEXP savvy_sk_draw_line__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__from_x, SEXP c_arg__from_y, SEXP c_arg__to_x, SEXP c_arg__to_y);
SEXP savvy_sk_draw_path__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat1, SEXP c_arg__props, SEXP c_arg__svg, SEXP c_arg__mat2);
SEXP savvy_sk_draw_png__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__png_bytes, SEXP c_arg__left_top);
SEXP savvy_sk_draw_points__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__mode);
SEXP savvy_sk_matrix_default__ffi(void);

// methods and associated functions for BlendMode


// methods and associated functions for Cap


// methods and associated functions for Join


// methods and associated functions for PaintProps
SEXP savvy_PaintProps_set_props__ffi(SEXP c_arg__color, SEXP c_arg__style, SEXP c_arg__join, SEXP c_arg__cap, SEXP c_arg__width, SEXP c_arg__miter, SEXP c_arg__blend_mode, SEXP c_arg__path_effect);

// methods and associated functions for PathEffect
SEXP savvy_PathEffect_get_label__ffi(SEXP self__);
SEXP savvy_PathEffect_no_effect__ffi(void);
SEXP savvy_PathEffect_trim__ffi(SEXP c_arg__start, SEXP c_arg__end);

// methods and associated functions for PointMode


// methods and associated functions for Style
