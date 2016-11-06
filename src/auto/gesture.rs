// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
#[cfg(feature = "v3_14")]
use EventSequenceState;
use ffi;
#[cfg(feature = "v3_14")]
use gdk;
use glib::object::IsA;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use std::mem;

glib_wrapper! {
    pub struct Gesture(Object<ffi::GtkGesture>): EventController;

    match fn {
        get_type => || ffi::gtk_gesture_get_type(),
    }
}

pub trait GestureExt {
    #[cfg(feature = "v3_14")]
    fn get_bounding_box(&self) -> Option<gdk::Rectangle>;

    #[cfg(feature = "v3_14")]
    fn get_bounding_box_center(&self) -> Option<(f64, f64)>;

    #[cfg(feature = "v3_14")]
    fn get_device(&self) -> Option<gdk::Device>;

    #[cfg(feature = "v3_14")]
    fn get_group(&self) -> Vec<Gesture>;

    //fn get_last_event(&self, sequence: /*Ignored*/&gdk::EventSequence) -> Option<gdk::Event>;

    //#[cfg(feature = "v3_14")]
    //fn get_last_updated_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence>;

    //#[cfg(feature = "v3_14")]
    //fn get_point(&self, sequence: /*Ignored*/Option<&gdk::EventSequence>) -> Option<(f64, f64)>;

    //#[cfg(feature = "v3_14")]
    //fn get_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence) -> EventSequenceState;

    //#[cfg(feature = "v3_14")]
    //fn get_sequences(&self) -> /*Ignored*/Vec<gdk::EventSequence>;

    #[cfg(feature = "v3_14")]
    fn get_window(&self) -> Option<gdk::Window>;

    #[cfg(feature = "v3_14")]
    fn group<T: IsA<Gesture>>(&self, gesture: &T);

    //#[cfg(feature = "v3_14")]
    //fn handles_sequence(&self, sequence: /*Ignored*/&gdk::EventSequence) -> bool;

    #[cfg(feature = "v3_14")]
    fn is_active(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn is_grouped_with<T: IsA<Gesture>>(&self, other: &T) -> bool;

    #[cfg(feature = "v3_14")]
    fn is_recognized(&self) -> bool;

    //#[cfg(feature = "v3_14")]
    //fn set_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence, state: EventSequenceState) -> bool;

    #[cfg(feature = "v3_14")]
    fn set_state(&self, state: EventSequenceState) -> bool;

    #[cfg(feature = "v3_14")]
    fn set_window(&self, window: Option<&gdk::Window>);

    #[cfg(feature = "v3_14")]
    fn ungroup(&self);

    //#[cfg(feature = "v3_14")]
    //fn connect_begin<Unsupported or ignored types>(&self, f: F) -> u64;

    //#[cfg(feature = "v3_14")]
    //fn connect_cancel<Unsupported or ignored types>(&self, f: F) -> u64;

    //#[cfg(feature = "v3_14")]
    //fn connect_end<Unsupported or ignored types>(&self, f: F) -> u64;

    //#[cfg(feature = "v3_14")]
    //fn connect_sequence_state_changed<Unsupported or ignored types>(&self, f: F) -> u64;

    //#[cfg(feature = "v3_14")]
    //fn connect_update<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<Gesture>> GestureExt for O {
    #[cfg(feature = "v3_14")]
    fn get_bounding_box(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_bounding_box_center(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box_center(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_device(&self) -> Option<gdk::Device> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_device(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_group(&self) -> Vec<Gesture> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_group(self.to_glib_none().0))
        }
    }

    //fn get_last_event(&self, sequence: /*Ignored*/&gdk::EventSequence) -> Option<gdk::Event> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_last_event() }
    //}

    //#[cfg(feature = "v3_14")]
    //fn get_last_updated_sequence(&self) -> /*Ignored*/Option<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_last_updated_sequence() }
    //}

    //#[cfg(feature = "v3_14")]
    //fn get_point(&self, sequence: /*Ignored*/Option<&gdk::EventSequence>) -> Option<(f64, f64)> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_point() }
    //}

    //#[cfg(feature = "v3_14")]
    //fn get_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence) -> EventSequenceState {
    //    unsafe { TODO: call ffi::gtk_gesture_get_sequence_state() }
    //}

    //#[cfg(feature = "v3_14")]
    //fn get_sequences(&self) -> /*Ignored*/Vec<gdk::EventSequence> {
    //    unsafe { TODO: call ffi::gtk_gesture_get_sequences() }
    //}

    #[cfg(feature = "v3_14")]
    fn get_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn group<T: IsA<Gesture>>(&self, gesture: &T) {
        unsafe {
            ffi::gtk_gesture_group(self.to_glib_none().0, gesture.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn handles_sequence(&self, sequence: /*Ignored*/&gdk::EventSequence) -> bool {
    //    unsafe { TODO: call ffi::gtk_gesture_handles_sequence() }
    //}

    #[cfg(feature = "v3_14")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_active(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn is_grouped_with<T: IsA<Gesture>>(&self, other: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_grouped_with(self.to_glib_none().0, other.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn is_recognized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_recognized(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn set_sequence_state(&self, sequence: /*Ignored*/&gdk::EventSequence, state: EventSequenceState) -> bool {
    //    unsafe { TODO: call ffi::gtk_gesture_set_sequence_state() }
    //}

    #[cfg(feature = "v3_14")]
    fn set_state(&self, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_state(self.to_glib_none().0, state.to_glib()))
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_window(&self, window: Option<&gdk::Window>) {
        unsafe {
            ffi::gtk_gesture_set_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn ungroup(&self) {
        unsafe {
            ffi::gtk_gesture_ungroup(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn connect_begin<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(feature = "v3_14")]
    //fn connect_cancel<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(feature = "v3_14")]
    //fn connect_end<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(feature = "v3_14")]
    //fn connect_sequence_state_changed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored sequence: Gdk.EventSequence
    //}

    //#[cfg(feature = "v3_14")]
    //fn connect_update<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored sequence: Gdk.EventSequence
    //}
}
