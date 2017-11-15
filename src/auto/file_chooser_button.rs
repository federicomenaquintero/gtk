// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Box;
use Container;
use Dialog;
use FileChooser;
use FileChooserAction;
use Orientable;
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
    pub struct FileChooserButton(Object<ffi::GtkFileChooserButton, ffi::GtkFileChooserButtonClass>): Box, Container, Widget, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_button_get_type(),
    }
}

impl FileChooserButton {
    pub fn new(title: &str, action: FileChooserAction) -> FileChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new(title.to_glib_none().0, action.to_glib())).downcast_unchecked()
        }
    }

    pub fn new_with_dialog<P: IsA<Dialog>>(dialog: &P) -> FileChooserButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new_with_dialog(dialog.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait FileChooserButtonExt {
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    fn get_width_chars(&self) -> i32;

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_title(&self, title: &str);

    fn set_width_chars(&self, n_chars: i32);

    fn connect_file_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dialog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserButton> + IsA<glib::object::Object>> FileChooserButtonExt for O {
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_button_get_focus_on_click(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_button_get_title(self.to_glib_none().0))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_file_chooser_button_get_width_chars(self.to_glib_none().0)
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_file_chooser_button_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_file_chooser_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_file_chooser_button_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    fn connect_file_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "file-set",
                transmute(file_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_dialog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::dialog",
                transmute(notify_dialog_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width-chars",
                transmute(notify_width_chars_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn file_set_trampoline<P>(this: *mut ffi::GtkFileChooserButton, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_dialog_trampoline<P>(this: *mut ffi::GtkFileChooserButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::GtkFileChooserButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_width_chars_trampoline<P>(this: *mut ffi::GtkFileChooserButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).downcast_unchecked())
}
