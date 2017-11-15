// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use CellEditable;
use Editable;
use Entry;
use Widget;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry, ffi::GtkSearchEntryClass>): Entry, Widget, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SearchEntryExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_next_match(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_previous_match(&self);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_stop_search(&self);
}

impl<O: IsA<SearchEntry> + IsA<glib::object::Object> + glib::object::ObjectExt> SearchEntryExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_entry_handle_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-match",
                transmute(next_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_next_match(&self) {
        let _ = self.emit("next-match", &[]).unwrap();
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "previous-match",
                transmute(previous_match_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_previous_match(&self) {
        let _ = self.emit("previous-match", &[]).unwrap();
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-changed",
                transmute(search_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stop-search",
                transmute(stop_search_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn emit_stop_search(&self) {
        let _ = self.emit("stop-search", &[]).unwrap();
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn next_match_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn previous_match_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn search_changed_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn stop_search_trampoline<P>(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer)
where P: IsA<SearchEntry> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchEntry::from_glib_borrow(this).downcast_unchecked())
}
