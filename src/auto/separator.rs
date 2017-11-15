// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Separator(Object<ffi::GtkSeparator, ffi::GtkSeparatorClass>): Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_separator_get_type(),
    }
}

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_new(orientation.to_glib())).downcast_unchecked()
        }
    }
}
