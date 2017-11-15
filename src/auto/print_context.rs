// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use PageSetup;
use cairo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PrintContext(Object<ffi::GtkPrintContext>);

    match fn {
        get_type => || ffi::gtk_print_context_get_type(),
    }
}

pub trait PrintContextExt {
    fn create_pango_context(&self) -> Option<pango::Context>;

    fn create_pango_layout(&self) -> Option<pango::Layout>;

    fn get_cairo_context(&self) -> Option<cairo::Context>;

    fn get_dpi_x(&self) -> f64;

    fn get_dpi_y(&self) -> f64;

    fn get_hard_margins(&self) -> Option<(f64, f64, f64, f64)>;

    fn get_height(&self) -> f64;

    fn get_page_setup(&self) -> Option<PageSetup>;

    fn get_pango_fontmap(&self) -> Option<pango::FontMap>;

    fn get_width(&self) -> f64;

    fn set_cairo_context(&self, cr: &cairo::Context, dpi_x: f64, dpi_y: f64);
}

impl<O: IsA<PrintContext>> PrintContextExt for O {
    fn create_pango_context(&self) -> Option<pango::Context> {
        unsafe {
            from_glib_full(ffi::gtk_print_context_create_pango_context(self.to_glib_none().0))
        }
    }

    fn create_pango_layout(&self) -> Option<pango::Layout> {
        unsafe {
            from_glib_full(ffi::gtk_print_context_create_pango_layout(self.to_glib_none().0))
        }
    }

    fn get_cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gtk_print_context_get_cairo_context(self.to_glib_none().0))
        }
    }

    fn get_dpi_x(&self) -> f64 {
        unsafe {
            ffi::gtk_print_context_get_dpi_x(self.to_glib_none().0)
        }
    }

    fn get_dpi_y(&self) -> f64 {
        unsafe {
            ffi::gtk_print_context_get_dpi_y(self.to_glib_none().0)
        }
    }

    fn get_hard_margins(&self) -> Option<(f64, f64, f64, f64)> {
        unsafe {
            let mut top = mem::uninitialized();
            let mut bottom = mem::uninitialized();
            let mut left = mem::uninitialized();
            let mut right = mem::uninitialized();
            let ret = from_glib(ffi::gtk_print_context_get_hard_margins(self.to_glib_none().0, &mut top, &mut bottom, &mut left, &mut right));
            if ret { Some((top, bottom, left, right)) } else { None }
        }
    }

    fn get_height(&self) -> f64 {
        unsafe {
            ffi::gtk_print_context_get_height(self.to_glib_none().0)
        }
    }

    fn get_page_setup(&self) -> Option<PageSetup> {
        unsafe {
            from_glib_none(ffi::gtk_print_context_get_page_setup(self.to_glib_none().0))
        }
    }

    fn get_pango_fontmap(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_none(ffi::gtk_print_context_get_pango_fontmap(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> f64 {
        unsafe {
            ffi::gtk_print_context_get_width(self.to_glib_none().0)
        }
    }

    fn set_cairo_context(&self, cr: &cairo::Context, dpi_x: f64, dpi_y: f64) {
        unsafe {
            ffi::gtk_print_context_set_cairo_context(self.to_glib_none().0, mut_override(cr.to_glib_none().0), dpi_x, dpi_y);
        }
    }
}
