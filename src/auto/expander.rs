// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Expander(Object<ffi::GtkExpander, ffi::GtkExpanderClass>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_expander_get_type(),
    }
}

impl Expander {
    pub fn new<'a, P: Into<Option<&'a str>>>(label: P) -> Expander {
        assert_initialized_main_thread!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new(label.0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ExpanderExt {
    fn get_expanded(&self) -> bool;

    fn get_label(&self) -> Option<String>;

    fn get_label_fill(&self) -> bool;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_resize_toplevel(&self) -> bool;

    fn get_spacing(&self) -> i32;

    fn get_use_markup(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn set_expanded(&self, expanded: bool);

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    fn set_label_fill(&self, label_fill: bool);

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q);

    fn set_resize_toplevel(&self, resize_toplevel: bool);

    fn set_spacing(&self, spacing: i32);

    fn set_use_markup(&self, use_markup: bool);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Expander> + IsA<glib::object::Object> + glib::object::ObjectExt> ExpanderExt for O {
    fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_expanded(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_fill(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_label_fill(self.to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_resize_toplevel(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_resize_toplevel(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(self.to_glib_none().0)
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_markup(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_underline(self.to_glib_none().0))
        }
    }

    fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_expander_set_expanded(self.to_glib_none().0, expanded.to_glib());
        }
    }

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::gtk_expander_set_label(self.to_glib_none().0, label.0);
        }
    }

    fn set_label_fill(&self, label_fill: bool) {
        unsafe {
            ffi::gtk_expander_set_label_fill(self.to_glib_none().0, label_fill.to_glib());
        }
    }

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q) {
        let label_widget = label_widget.into();
        let label_widget = label_widget.to_glib_none();
        unsafe {
            ffi::gtk_expander_set_label_widget(self.to_glib_none().0, label_widget.0);
        }
    }

    fn set_resize_toplevel(&self, resize_toplevel: bool) {
        unsafe {
            ffi::gtk_expander_set_resize_toplevel(self.to_glib_none().0, resize_toplevel.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_expander_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::gtk_expander_set_use_markup(self.to_glib_none().0, use_markup.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_expander_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate(&self) {
        let _ = self.emit("activate", &[]).unwrap();
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::expanded",
                transmute(notify_expanded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-fill",
                transmute(notify_label_fill_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label-widget",
                transmute(notify_label_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::resize-toplevel",
                transmute(notify_resize_toplevel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::spacing",
                transmute(notify_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-markup",
                transmute(notify_use_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-underline",
                transmute(notify_use_underline_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkExpander, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_expanded_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_fill_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_widget_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_resize_toplevel_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_spacing_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_markup_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_underline_trampoline<P>(this: *mut ffi::GtkExpander, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Expander> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Expander::from_glib_borrow(this).downcast_unchecked())
}
