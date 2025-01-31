/* automatically generated by rust-bindgen 0.64.0 */

extern "C" {
    pub fn Fl_set_color_int(c: cty::c_uint);
}
extern "C" {
    pub fn Fl_set_color_rgb(r: cty::c_uchar, g: cty::c_uchar, b: cty::c_uchar);
}
extern "C" {
    pub fn Fl_get_color() -> cty::c_uint;
}
extern "C" {
    pub fn Fl_push_clip(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int);
}
extern "C" {
    pub fn Fl_push_no_clip();
}
extern "C" {
    pub fn Fl_pop_clip();
}
extern "C" {
    pub fn Fl_not_clipped(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int)
        -> cty::c_int;
}
extern "C" {
    pub fn Fl_clip_box(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        X: *mut cty::c_int,
        Y: *mut cty::c_int,
        W: *mut cty::c_int,
        H: *mut cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_restore_clip();
}
extern "C" {
    pub fn Fl_set_clip_region(r: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_clip_region() -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_point(x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_line_style(style: cty::c_int, width: cty::c_int, dashes: *mut cty::c_char);
}
extern "C" {
    pub fn Fl_rect(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int);
}
extern "C" {
    pub fn Fl_focus_rect(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int);
}
extern "C" {
    pub fn Fl_rect_with_color(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        c: cty::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int);
}
extern "C" {
    pub fn Fl_rectf_with_color(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        c: cty::c_uint,
    );
}
extern "C" {
    pub fn Fl_rectf_with_rgb(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        r: cty::c_uchar,
        g: cty::c_uchar,
        b: cty::c_uchar,
    );
}
extern "C" {
    pub fn Fl_line(x: cty::c_int, y: cty::c_int, x1: cty::c_int, y1: cty::c_int);
}
extern "C" {
    pub fn Fl_line2(
        x: cty::c_int,
        y: cty::c_int,
        x1: cty::c_int,
        y1: cty::c_int,
        x2: cty::c_int,
        y2: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_loop(
        x: cty::c_int,
        y: cty::c_int,
        x1: cty::c_int,
        y1: cty::c_int,
        x2: cty::c_int,
        y2: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_loop2(
        x: cty::c_int,
        y: cty::c_int,
        x1: cty::c_int,
        y1: cty::c_int,
        x2: cty::c_int,
        y2: cty::c_int,
        x3: cty::c_int,
        y3: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon(
        x: cty::c_int,
        y: cty::c_int,
        x1: cty::c_int,
        y1: cty::c_int,
        x2: cty::c_int,
        y2: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_polygon2(
        x: cty::c_int,
        y: cty::c_int,
        x1: cty::c_int,
        y1: cty::c_int,
        x2: cty::c_int,
        y2: cty::c_int,
        x3: cty::c_int,
        y3: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_xyline(x: cty::c_int, y: cty::c_int, x1: cty::c_int);
}
extern "C" {
    pub fn Fl_xyline2(x: cty::c_int, y: cty::c_int, x1: cty::c_int, y2: cty::c_int);
}
extern "C" {
    pub fn Fl_xyline3(x: cty::c_int, y: cty::c_int, x1: cty::c_int, y2: cty::c_int, x3: cty::c_int);
}
extern "C" {
    pub fn Fl_yxline(x: cty::c_int, y: cty::c_int, y1: cty::c_int);
}
extern "C" {
    pub fn Fl_yxline2(x: cty::c_int, y: cty::c_int, y1: cty::c_int, x2: cty::c_int);
}
extern "C" {
    pub fn Fl_yxline3(x: cty::c_int, y: cty::c_int, y1: cty::c_int, x2: cty::c_int, y3: cty::c_int);
}
extern "C" {
    pub fn Fl_arc(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int, a1: f64, a2: f64);
}
extern "C" {
    pub fn Fl_pie(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int, a1: f64, a2: f64);
}
extern "C" {
    pub fn Fl_push_matrix();
}
extern "C" {
    pub fn Fl_pop_matrix();
}
extern "C" {
    pub fn Fl_scale(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_scale2(x: f64);
}
extern "C" {
    pub fn Fl_translate(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_rotate(d: f64);
}
extern "C" {
    pub fn Fl_mult_matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64);
}
extern "C" {
    pub fn Fl_begin_points();
}
extern "C" {
    pub fn Fl_begin_line();
}
extern "C" {
    pub fn Fl_begin_loop();
}
extern "C" {
    pub fn Fl_begin_polygon();
}
extern "C" {
    pub fn Fl_vertex(x: f64, y: f64);
}
extern "C" {
    pub fn Fl_curve(X0: f64, Y0: f64, X1: f64, Y1: f64, X2: f64, Y2: f64, X3: f64, Y3: f64);
}
extern "C" {
    pub fn Fl_arc2(x: f64, y: f64, r: f64, start: f64, end: f64);
}
extern "C" {
    pub fn Fl_circle(x: f64, y: f64, r: f64);
}
extern "C" {
    pub fn Fl_draw_circle(x: cty::c_int, y: cty::c_int, d: cty::c_int, c: cty::c_uint);
}
extern "C" {
    pub fn Fl_end_points();
}
extern "C" {
    pub fn Fl_end_line();
}
extern "C" {
    pub fn Fl_end_loop();
}
extern "C" {
    pub fn Fl_end_polygon();
}
extern "C" {
    pub fn Fl_begin_complex_polygon();
}
extern "C" {
    pub fn Fl_gap();
}
extern "C" {
    pub fn Fl_end_complex_polygon();
}
extern "C" {
    pub fn Fl_transform_x(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_y(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dx(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transform_dy(x: f64, y: f64) -> f64;
}
extern "C" {
    pub fn Fl_transformed_vertex(xf: f64, yf: f64);
}
extern "C" {
    pub fn Fl_end_offscreen();
}
extern "C" {
    pub fn Fl_set_draw_font(face: cty::c_int, fsize: cty::c_int);
}
extern "C" {
    pub fn Fl_font() -> cty::c_int;
}
extern "C" {
    pub fn Fl_size() -> cty::c_int;
}
extern "C" {
    pub fn Fl_height() -> cty::c_int;
}
extern "C" {
    pub fn Fl_set_height(font: cty::c_int, size: cty::c_int) -> cty::c_int;
}
extern "C" {
    pub fn Fl_descent() -> cty::c_int;
}
extern "C" {
    pub fn Fl_width(txt: *const cty::c_char) -> f64;
}
extern "C" {
    pub fn Fl_width2(txt: *const cty::c_char, n: cty::c_int) -> f64;
}
extern "C" {
    pub fn Fl_width3(c: cty::c_uint) -> f64;
}
extern "C" {
    pub fn Fl_text_extents(
        arg1: *const cty::c_char,
        dx: *mut cty::c_int,
        dy: *mut cty::c_int,
        w: *mut cty::c_int,
        h: *mut cty::c_int,
    );
}
extern "C" {
    pub fn Fl_text_extents2(
        t: *const cty::c_char,
        n: cty::c_int,
        dx: *mut cty::c_int,
        dy: *mut cty::c_int,
        w: *mut cty::c_int,
        h: *mut cty::c_int,
    );
}
extern "C" {
    pub fn Fl_latin1_to_local(t: *const cty::c_char, n: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_local_to_latin1(t: *const cty::c_char, n: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_mac_roman_to_local(t: *const cty::c_char, n: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_local_to_mac_roman(t: *const cty::c_char, n: cty::c_int) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_draw(str_: *const cty::c_char, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_draw2(angle: cty::c_int, str_: *const cty::c_char, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_draw3(str_: *const cty::c_char, n: cty::c_int, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_draw4(
        angle: cty::c_int,
        str_: *const cty::c_char,
        n: cty::c_int,
        x: cty::c_int,
        y: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_rtl_draw(str_: *const cty::c_char, n: cty::c_int, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_measure(
        str_: *const cty::c_char,
        x: *mut cty::c_int,
        y: *mut cty::c_int,
        draw_symbols: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_draw5(
        str_: *const cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        align: cty::c_int,
        img: *mut *mut cty::c_void,
        draw_symbols: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_frame(
        s: *const cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_frame2(
        s: *const cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box(
        box_type: cty::c_int,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        arg1: cty::c_uint,
    );
}
extern "C" {
    pub fn Fl_draw_image(
        buf: *const cty::c_uchar,
        X: cty::c_int,
        Y: cty::c_int,
        W: cty::c_int,
        H: cty::c_int,
        D: cty::c_int,
        L: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_image_mono(
        buf: *const cty::c_uchar,
        X: cty::c_int,
        Y: cty::c_int,
        W: cty::c_int,
        H: cty::c_int,
        D: cty::c_int,
        L: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_can_do_alpha_blending() -> cty::c_char;
}
extern "C" {
    pub fn Fl_read_image(
        p: *mut cty::c_uchar,
        X: cty::c_int,
        Y: cty::c_int,
        W: cty::c_int,
        H: cty::c_int,
        alpha: cty::c_int,
    ) -> *mut cty::c_uchar;
}
extern "C" {
    pub fn Fl_capture_window_part(
        win: *mut cty::c_void,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
    ) -> *mut cty::c_uchar;
}
extern "C" {
    pub fn Fl_draw_pixmap(
        data: *const *const cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        bg: cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_draw_pixmap2(
        data: *const *mut cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        bg: cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap(
        data: *const *mut cty::c_char,
        w: *mut cty::c_int,
        h: *mut cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_measure_pixmap2(
        cdata: *const *const cty::c_char,
        w: *mut cty::c_int,
        h: *mut cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_shortcut_label(shortcut: cty::c_uint) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_shortcut_label2(
        shortcut: cty::c_uint,
        eom: *mut *const cty::c_char,
    ) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_old_shortcut(s: *const cty::c_char) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_overlay_rect(x: cty::c_int, y: cty::c_int, w: cty::c_int, h: cty::c_int);
}
extern "C" {
    pub fn Fl_overlay_clear();
}
extern "C" {
    pub fn Fl_set_cursor(cursor: cty::c_int);
}
extern "C" {
    pub fn Fl_set_cursor2(cursor: cty::c_int, fg: cty::c_int, bg: cty::c_int);
}
extern "C" {
    pub fn Fl_expand_text(
        from: *const cty::c_char,
        buf: *mut cty::c_char,
        maxbuf: cty::c_int,
        maxw: f64,
        n: *mut cty::c_int,
        width: *mut f64,
        wrap: cty::c_int,
        draw_symbols: cty::c_int,
    ) -> *const cty::c_char;
}
extern "C" {
    pub fn Fl_set_status(X: cty::c_int, Y: cty::c_int, W: cty::c_int, H: cty::c_int);
}
extern "C" {
    pub fn Fl_set_spot(
        font: cty::c_int,
        size: cty::c_int,
        X: cty::c_int,
        Y: cty::c_int,
        W: cty::c_int,
        H: cty::c_int,
        win: *mut cty::c_void,
    );
}
extern "C" {
    pub fn Fl_reset_spot();
}
extern "C" {
    pub fn Fl_show_colormap(old_col: cty::c_uint) -> cty::c_uint;
}
extern "C" {
    pub fn Fl_copy_offscreen(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        pixmap: *mut cty::c_void,
        srcx: cty::c_int,
        srcy: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_create_offscreen(w: cty::c_int, h: cty::c_int) -> *mut cty::c_void;
}
extern "C" {
    pub fn Fl_begin_offscreen(b: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_delete_offscreen(bitmap: *mut cty::c_void);
}
extern "C" {
    pub fn Fl_rescale_offscreen(ctx: *mut *mut cty::c_void);
}
extern "C" {
    pub fn Fl_draw_text2(
        str_: *const cty::c_char,
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        align: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_check(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        col: cty::c_uint,
    );
}
extern "C" {
    pub fn Fl_add_symbol(
        name: *const cty::c_char,
        drawit: ::core::option::Option<unsafe extern "C" fn(arg1: cty::c_uint)>,
        scalable: cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_rounded_rect(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        r: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_rounded_rectf(
        x: cty::c_int,
        y: cty::c_int,
        w: cty::c_int,
        h: cty::c_int,
        r: cty::c_int,
    );
}
