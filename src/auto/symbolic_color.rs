// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct SymbolicColor(Shared<ffi::GtkSymbolicColor>);

    match fn {
        ref => |ptr| ffi::gtk_symbolic_color_ref(ptr),
        unref => |ptr| ffi::gtk_symbolic_color_unref(ptr),
    }
}

impl SymbolicColor {
    pub fn new_alpha(color: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_alpha(color.to_glib_none().0, factor))
        }
    }

    //pub fn new_literal(color: /*Ignored*/&gdk::RGBA) -> SymbolicColor {
    //    unsafe { TODO: call ffi::gtk_symbolic_color_new_literal() }
    //}

    pub fn new_mix(color1: &SymbolicColor, color2: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_mix(color1.to_glib_none().0, color2.to_glib_none().0, factor))
        }
    }

    pub fn new_name(name: &str) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_name(name.to_glib_none().0))
        }
    }

    pub fn new_shade(color: &SymbolicColor, factor: f64) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_shade(color.to_glib_none().0, factor))
        }
    }

    pub fn new_win32(theme_class: &str, id: i32) -> SymbolicColor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_new_win32(theme_class.to_glib_none().0, id))
        }
    }

    //pub fn resolve(&self, props: Option<&StyleProperties>, resolved_color: /*Ignored*/gdk::RGBA) -> bool {
    //    unsafe { TODO: call ffi::gtk_symbolic_color_resolve() }
    //}

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_symbolic_color_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for SymbolicColor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}