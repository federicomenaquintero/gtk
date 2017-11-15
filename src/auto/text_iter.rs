// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use TextBuffer;
use TextChildAnchor;
use TextMark;
use TextSearchFlags;
use TextTag;
use ffi;
use gdk_pixbuf;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use std::cmp;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TextIter(Boxed<ffi::GtkTextIter>);

    match fn {
        copy => |ptr| ffi::gtk_text_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_text_iter_free(ptr),
        get_type => || ffi::gtk_text_iter_get_type(),
    }
}

impl TextIter {
    pub fn assign(&mut self, other: &TextIter) {
        unsafe {
            ffi::gtk_text_iter_assign(self.to_glib_none_mut().0, other.to_glib_none().0);
        }
    }

    pub fn backward_char(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_char(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_chars(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    //pub fn backward_find_char<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a TextIter>>>(&mut self, pred: /*Unknown conversion*//*Unimplemented*/TextCharPredicate, user_data: P, limit: Q) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_iter_backward_find_char() }
    //}

    pub fn backward_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_line(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_search<'a, P: Into<Option<&'a TextIter>>>(&self, str: &str, flags: TextSearchFlags, limit: P) -> Option<(TextIter, TextIter)> {
        let limit = limit.into();
        let limit = limit.to_glib_none();
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_iter_backward_search(self.to_glib_none().0, str.to_glib_none().0, flags.to_glib(), match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, limit.0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    pub fn backward_sentence_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_sentence_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_sentence_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_sentence_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_to_tag_toggle<'a, P: Into<Option<&'a TextTag>>>(&mut self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_to_tag_toggle(self.to_glib_none_mut().0, tag.0))
        }
    }

    pub fn backward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_line(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_visible_word_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_word_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_visible_word_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_word_start(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_word_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_backward_word_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn begins_tag<'a, P: Into<Option<&'a TextTag>>>(&self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_begins_tag(self.to_glib_none().0, tag.0))
        }
    }

    pub fn can_insert(&self, default_editability: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_can_insert(self.to_glib_none().0, default_editability.to_glib()))
        }
    }

    fn compare(&self, rhs: &TextIter) -> i32 {
        unsafe {
            ffi::gtk_text_iter_compare(self.to_glib_none().0, rhs.to_glib_none().0)
        }
    }

    pub fn editable(&self, default_setting: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_editable(self.to_glib_none().0, default_setting.to_glib()))
        }
    }

    pub fn ends_line(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_ends_line(self.to_glib_none().0))
        }
    }

    pub fn ends_sentence(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_ends_sentence(self.to_glib_none().0))
        }
    }

    pub fn ends_tag<'a, P: Into<Option<&'a TextTag>>>(&self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_ends_tag(self.to_glib_none().0, tag.0))
        }
    }

    pub fn ends_word(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_ends_word(self.to_glib_none().0))
        }
    }

    fn equal(&self, rhs: &TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_equal(self.to_glib_none().0, rhs.to_glib_none().0))
        }
    }

    pub fn forward_char(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_char(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_chars(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    //pub fn forward_find_char<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a TextIter>>>(&mut self, pred: /*Unknown conversion*//*Unimplemented*/TextCharPredicate, user_data: P, limit: Q) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_iter_forward_find_char() }
    //}

    pub fn forward_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_line(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_search<'a, P: Into<Option<&'a TextIter>>>(&self, str: &str, flags: TextSearchFlags, limit: P) -> Option<(TextIter, TextIter)> {
        let limit = limit.into();
        let limit = limit.to_glib_none();
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_iter_forward_search(self.to_glib_none().0, str.to_glib_none().0, flags.to_glib(), match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, limit.0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    pub fn forward_sentence_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_sentence_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_sentence_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_sentence_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_to_end(&mut self) {
        unsafe {
            ffi::gtk_text_iter_forward_to_end(self.to_glib_none_mut().0);
        }
    }

    pub fn forward_to_line_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_to_line_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_to_tag_toggle<'a, P: Into<Option<&'a TextTag>>>(&mut self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_to_tag_toggle(self.to_glib_none_mut().0, tag.0))
        }
    }

    pub fn forward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_line(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_visible_word_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_word_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_visible_word_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_word_end(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_word_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_forward_word_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_buffer(self.to_glib_none().0))
        }
    }

    pub fn get_bytes_in_line(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_bytes_in_line(self.to_glib_none().0)
        }
    }

    pub fn get_char(&self) -> Option<char> {
        unsafe {
            from_glib(ffi::gtk_text_iter_get_char(self.to_glib_none().0))
        }
    }

    pub fn get_chars_in_line(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_chars_in_line(self.to_glib_none().0)
        }
    }

    pub fn get_child_anchor(&self) -> Option<TextChildAnchor> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_child_anchor(self.to_glib_none().0))
        }
    }

    pub fn get_language(&self) -> Option<pango::Language> {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_language(self.to_glib_none().0))
        }
    }

    pub fn get_line(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_line(self.to_glib_none().0)
        }
    }

    pub fn get_line_index(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_line_index(self.to_glib_none().0)
        }
    }

    pub fn get_line_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_line_offset(self.to_glib_none().0)
        }
    }

    pub fn get_marks(&self) -> Vec<TextMark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_marks(self.to_glib_none().0))
        }
    }

    pub fn get_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_offset(self.to_glib_none().0)
        }
    }

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_text_iter_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_slice(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_slice(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_tags(&self) -> Vec<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_tags(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_text(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_toggled_tags(&self, toggled_on: bool) -> Vec<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_text_iter_get_toggled_tags(self.to_glib_none().0, toggled_on.to_glib()))
        }
    }

    pub fn get_visible_line_index(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_visible_line_index(self.to_glib_none().0)
        }
    }

    pub fn get_visible_line_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_text_iter_get_visible_line_offset(self.to_glib_none().0)
        }
    }

    pub fn get_visible_slice(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_visible_slice(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_visible_text(&self, end: &TextIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_iter_get_visible_text(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn has_tag(&self, tag: &TextTag) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_has_tag(self.to_glib_none().0, tag.to_glib_none().0))
        }
    }

    pub fn in_range(&self, start: &TextIter, end: &TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_in_range(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn inside_sentence(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_inside_sentence(self.to_glib_none().0))
        }
    }

    pub fn inside_word(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_inside_word(self.to_glib_none().0))
        }
    }

    pub fn is_cursor_position(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_is_cursor_position(self.to_glib_none().0))
        }
    }

    pub fn is_end(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_is_end(self.to_glib_none().0))
        }
    }

    pub fn is_start(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_is_start(self.to_glib_none().0))
        }
    }

    pub fn order(&mut self, second: &mut TextIter) {
        unsafe {
            ffi::gtk_text_iter_order(self.to_glib_none_mut().0, second.to_glib_none_mut().0);
        }
    }

    pub fn set_line(&mut self, line_number: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line(self.to_glib_none_mut().0, line_number);
        }
    }

    pub fn set_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    pub fn set_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    pub fn set_offset(&mut self, char_offset: i32) {
        unsafe {
            ffi::gtk_text_iter_set_offset(self.to_glib_none_mut().0, char_offset);
        }
    }

    pub fn set_visible_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_visible_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    pub fn set_visible_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            ffi::gtk_text_iter_set_visible_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    pub fn starts_line(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_starts_line(self.to_glib_none().0))
        }
    }

    pub fn starts_sentence(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_starts_sentence(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn starts_tag<'a, P: Into<Option<&'a TextTag>>>(&self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_starts_tag(self.to_glib_none().0, tag.0))
        }
    }

    pub fn starts_word(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_starts_word(self.to_glib_none().0))
        }
    }

    pub fn toggles_tag<'a, P: Into<Option<&'a TextTag>>>(&self, tag: P) -> bool {
        let tag = tag.into();
        let tag = tag.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_text_iter_toggles_tag(self.to_glib_none().0, tag.0))
        }
    }
}

impl PartialOrd for TextIter {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TextIter {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for TextIter {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TextIter {}
