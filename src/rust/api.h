SEXP savvy_sk_absolute_fill__ffi(SEXP c_arg__size, SEXP c_arg__fill);
SEXP savvy_sk_as_png__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat);
SEXP savvy_sk_draw_atlas__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__png_bytes, SEXP c_arg__scale, SEXP c_arg__radians, SEXP c_arg__tx, SEXP c_arg__ty, SEXP c_arg__anchor_x, SEXP c_arg__anchor_y);
SEXP savvy_sk_draw_circle__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__radius);
SEXP savvy_sk_draw_diff_rect__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__outer_left, SEXP c_arg__outer_top, SEXP c_arg__outer_right, SEXP c_arg__outer_bottom, SEXP c_arg__outer_rx, SEXP c_arg__outer_ry, SEXP c_arg__inner_left, SEXP c_arg__inner_top, SEXP c_arg__inner_right, SEXP c_arg__inner_bottom, SEXP c_arg__inner_rx, SEXP c_arg__inner_ry);
SEXP savvy_sk_draw_line__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__from_x, SEXP c_arg__from_y, SEXP c_arg__to_x, SEXP c_arg__to_y);
SEXP savvy_sk_draw_path__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat1, SEXP c_arg__props, SEXP c_arg__svg, SEXP c_arg__mat2, SEXP c_arg__fill_type);
SEXP savvy_sk_draw_png__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__png_bytes, SEXP c_arg__left_top);
SEXP savvy_sk_draw_points__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__mode);
SEXP savvy_sk_draw_rounded_rect__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__left, SEXP c_arg__top, SEXP c_arg__right, SEXP c_arg__bottom, SEXP c_arg__rx, SEXP c_arg__ry);
SEXP savvy_sk_draw_text__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__text);
SEXP savvy_sk_draw_textblob__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__text, SEXP c_arg__x, SEXP c_arg__y);
SEXP savvy_sk_draw_textpath__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat1, SEXP c_arg__props, SEXP c_arg__text, SEXP c_arg__svg, SEXP c_arg__mat2);
SEXP savvy_sk_draw_vertices__ffi(SEXP c_arg__size, SEXP c_arg__curr_bytes, SEXP c_arg__mat, SEXP c_arg__props, SEXP c_arg__x, SEXP c_arg__y, SEXP c_arg__mode);
SEXP savvy_sk_get_text_width__ffi(SEXP c_arg__text, SEXP c_arg__props);
SEXP savvy_sk_list_families__ffi(void);
SEXP savvy_sk_matrix_map_point__ffi(SEXP c_arg__src_x, SEXP c_arg__src_y, SEXP c_arg__dst_x, SEXP c_arg__dst_y);
SEXP savvy_sk_path_bounds__ffi(SEXP c_arg__svg);
SEXP savvy_sk_path_interpolate__ffi(SEXP c_arg__value, SEXP c_arg__first, SEXP c_arg__second);
SEXP savvy_sk_path_transform__ffi(SEXP c_arg__svg, SEXP c_arg__mat);

// methods and associated functions for BlendMode


// methods and associated functions for Cap


// methods and associated functions for FillType


// methods and associated functions for FontStyle


// methods and associated functions for Join


// methods and associated functions for PaintAttrs
SEXP savvy_PaintAttrs_set_attrs__ffi(SEXP c_arg__color, SEXP c_arg__style, SEXP c_arg__join, SEXP c_arg__cap, SEXP c_arg__width, SEXP c_arg__miter, SEXP c_arg__fontsize, SEXP c_arg__family, SEXP c_arg__fontface, SEXP c_arg__blend_mode, SEXP c_arg__path_effect, SEXP c_arg__shader);

// methods and associated functions for PathEffect
SEXP savvy_PathEffect_corner__ffi(SEXP c_arg__radius);
SEXP savvy_PathEffect_dash__ffi(SEXP c_arg__intervals, SEXP c_arg__phase);
SEXP savvy_PathEffect_discrete__ffi(SEXP c_arg__length, SEXP c_arg__deviation, SEXP c_arg__seed);
SEXP savvy_PathEffect_get_label__ffi(SEXP self__);
SEXP savvy_PathEffect_line_2d__ffi(SEXP c_arg__width, SEXP c_arg__transform);
SEXP savvy_PathEffect_no_effect__ffi(void);
SEXP savvy_PathEffect_path_1d__ffi(SEXP c_arg__path, SEXP c_arg__advance, SEXP c_arg__phase, SEXP c_arg__style);
SEXP savvy_PathEffect_path_2d__ffi(SEXP c_arg__path, SEXP c_arg__transform);
SEXP savvy_PathEffect_sum__ffi(SEXP c_arg__first, SEXP c_arg__second);
SEXP savvy_PathEffect_trim__ffi(SEXP c_arg__start, SEXP c_arg__end);

// methods and associated functions for PointMode


// methods and associated functions for Shader
SEXP savvy_Shader_blend__ffi(SEXP c_arg__mode, SEXP c_arg__dst, SEXP c_arg__src);
SEXP savvy_Shader_color__ffi(SEXP c_arg__rgba);
SEXP savvy_Shader_conical_gradient__ffi(SEXP c_arg__start, SEXP c_arg__end, SEXP c_arg__radii, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform);
SEXP savvy_Shader_fractal_noise__ffi(SEXP c_arg__freq, SEXP c_arg__octaves, SEXP c_arg__seed, SEXP c_arg__tile_size);
SEXP savvy_Shader_from_picture__ffi(SEXP c_arg__img, SEXP c_arg__mode, SEXP c_arg__tile_size, SEXP c_arg__transform);
SEXP savvy_Shader_from_png__ffi(SEXP c_arg__png_bytes, SEXP c_arg__mode, SEXP c_arg__transform);
SEXP savvy_Shader_get_label__ffi(SEXP self__);
SEXP savvy_Shader_linear_gradient__ffi(SEXP c_arg__start, SEXP c_arg__end, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform);
SEXP savvy_Shader_no_shader__ffi(void);
SEXP savvy_Shader_radial_gradient__ffi(SEXP c_arg__center, SEXP c_arg__radius, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform);
SEXP savvy_Shader_sweep_gradient__ffi(SEXP c_arg__center, SEXP c_arg__start_angle, SEXP c_arg__end_angle, SEXP c_arg__from, SEXP c_arg__to, SEXP c_arg__mode, SEXP c_arg__flags, SEXP c_arg__transform);
SEXP savvy_Shader_turbulence__ffi(SEXP c_arg__freq, SEXP c_arg__octaves, SEXP c_arg__seed, SEXP c_arg__tile_size);

// methods and associated functions for Style


// methods and associated functions for TileMode


// methods and associated functions for VertexMode

