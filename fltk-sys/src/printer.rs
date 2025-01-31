/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Printer {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Printer_new() -> *mut Fl_Printer;
}
extern "C" {
    pub fn Fl_Printer_delete(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_begin_job(
        self_: *mut Fl_Printer,
        pagecount: cty::c_int,
        frompage: *mut cty::c_int,
        topage: *mut cty::c_int,
        perr_message: *mut *mut cty::c_char,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Printer_begin_page(self_: *mut Fl_Printer) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Printer_printable_rect(
        self_: *mut Fl_Printer,
        w: *mut cty::c_int,
        h: *mut cty::c_int,
    ) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Printer_margins(
        self_: *mut Fl_Printer,
        left: *mut cty::c_int,
        top: *mut cty::c_int,
        right: *mut cty::c_int,
        bottom: *mut cty::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_origin(self_: *mut Fl_Printer, x: *mut cty::c_int, y: *mut cty::c_int);
}
extern "C" {
    pub fn Fl_Printer_set_origin(self_: *mut Fl_Printer, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_Printer_scale(self_: *mut Fl_Printer, scale_x: f32, scale_y: f32);
}
extern "C" {
    pub fn Fl_Printer_rotate(self_: *mut Fl_Printer, angle: f32);
}
extern "C" {
    pub fn Fl_Printer_translate(self_: *mut Fl_Printer, x: cty::c_int, y: cty::c_int);
}
extern "C" {
    pub fn Fl_Printer_untranslate(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_end_page(self_: *mut Fl_Printer) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Printer_end_job(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_set_current(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_is_current(self_: *mut Fl_Printer) -> cty::c_int;
}
extern "C" {
    pub fn Fl_Printer_print_widget(
        self_: *mut Fl_Printer,
        widget: *mut cty::c_void,
        delta_x: cty::c_int,
        delta_y: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_print_window(
        self_: *mut Fl_Printer,
        win: *mut cty::c_void,
        x_offset: cty::c_int,
        y_offset: cty::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_set_dialog_title(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_printer(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_range(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_copies(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_all(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_pages(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_from(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_to(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_properties(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_copyNo(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_print_button(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_cancel_button(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_print_to_file(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_title(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_pagesize(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_mode(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_use(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_save(msg: *const cty::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_cancel(msg: *const cty::c_char);
}
