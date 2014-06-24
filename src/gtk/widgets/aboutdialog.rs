// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use ffi;
use gtk::traits;
use std::str;

struct_Widget!(AboutDialog)

impl AboutDialog {
    pub fn new() -> Option<AboutDialog> {
        let tmp_pointer = unsafe { ffi::gtk_about_dialog_new() };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(traits::Widget::wrap(tmp_pointer))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        let name = unsafe { ffi::gtk_about_dialog_get_program_name(self.pointer)) };
        if name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(name) })
        }
    }

    pub fn set_program_name(&self, name: &str) -> () {
        unsafe {
            name.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_version(&self) -> Option<String> {
        let version = unsafe { ffi::gtk_about_dialog_get_version(self.pointer)) };

        if version.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(version) })
        }
    }

    pub fn set_version(&self, version: &str) -> () {
        unsafe {
            version.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        let copyright = unsafe { ffi::gtk_about_dialog_get_copyright(self.pointer)) };

        if copyright.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(copyright) })
        }
    }

    pub fn set_copyright(&self, copyright: &str) -> () {
        unsafe {
            copyright.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        let comments = unsafe { ffi::gtk_about_dialog_get_comments(self.pointer)) };

        if comments.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(comments) })
        }
    }

    pub fn set_comments(&self, comments: &str) -> () {
        unsafe {
            comments.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_license(&self) -> Option<String> {
        let license = unsafe { ffi::gtk_about_dialog_get_license(self.pointer)) };

        if license.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(license) })
        }
    }

    pub fn set_license(&self, license: &str) -> () {
        unsafe {
            license.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_license(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_wrap_license(&self) -> Gboolean {
        unsafe { ffi::gtk_about_dialog_get_wrap_license(GTK_ABOUT_DIALOG(self.get_widget())) }
    }

    pub fn set_wrap_license(&self, wrap_license: Gboolean) -> () {
        unsafe { ffi::gtk_about_dialog_set_wrap_license(GTK_ABOUT_DIALOG(self.get_widget()), wrap_license) }
    }

    pub fn get_license_type(&self) -> License {
        unsafe { ffi::gtk_about_dialog_get_license_type(GTK_ABOUT_DIALOG(self.get_widget())) }
    }

    pub fn set_license_type(&self, license_type: License) -> () {
        unsafe { ffi::gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(self.get_widget()), license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        let website = unsafe { ffi::gtk_about_dialog_get_website(self.pointer)) };

        if website.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(website) })
        }
    }

    pub fn set_website(&self, website: &str) -> () {
        unsafe {
            website.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        let website_label = unsafe { ffi::gtk_about_dialog_get_website_label(self.pointer)) };

        if website_label.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(website_label) })
        }
    }

    pub fn set_website_label(&self, website_label: &str) -> () {
        unsafe {
            website_label.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        let authors = unsafe { ffi::gtk_about_dialog_set_authors(self.pointer)) };
        let mut ret = Vec::new();

        if !authors.is_null() {
            let mut it = 0;

            loop {
                let tmp = authors.offset(it);

                if tmp.is_null() {
                    break;
                }
                ret.push(str::raw::from_c_str(*tmp));
                it += 1;
            }
        }
        ret
    }

    pub fn set_authors(&self, authors: &Vec<String>) -> () {
        unsafe { ffi::gtk_about_dialog_set_authors(GTK_ABOUT_DIALOG(self.get_widget()), authors.as_slice().as_ptr()) }
    }

    pub fn get_artists(&self) -> Vec<String> {
        let artists = unsafe { ffi::gtk_about_dialog_get_artists(self.pointer)) };
        let mut ret = Vec::new();

        if !artists.is_null() {
            let mut it = 0;

            loop {
                let tmp = artists.offset(it);

                if tmp.is_null() {
                    break;
                }
                ret.push(str::raw::from_c_str(*tmp));
                it += 1;
            }
        }
        ret
    }

    pub fn set_artists(&self, artists: &Vec<String>) -> () {
        unsafe { ffi::gtk_about_dialog_set_artists(GTK_ABOUT_DIALOG(self.get_widget()), artists.as_slice().as_ptr()) }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        let documenters = unsafe { ffi::gtk_about_dialog_get_documenters(self.pointer)) };
        let mut ret = Vec::new();

        if !documenters.is_null() {
            let mut it = 0;

            loop {
                let tmp = documenters.offset(it);

                if tmp.is_null() {
                    break;
                }
                ret.push(str::raw::from_c_str(*tmp));
                it += 1;
            }
        }
        ret
    }

    pub fn set_documenters(&self, documenters: &Vec<String>) -> () {
        unsafe { ffi::gtk_about_dialog_set_documenters(GTK_ABOUT_DIALOG(self.get_widget()), documenters.as_slice().as_ptr()) }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        let translator_credits = unsafe { ffi::gtk_about_dialog_get_translator_credits(self.pointer)) };

        if translator_credits.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(translator_credits) })
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) -> () {
        unsafe {
            translator_credits.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_translator_credits(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { traits::Widget::wrap(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) -> () {
        unsafe { ffi::gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(self.get_widget()), GDK_PIXBUF(logo.get_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        let logo_icon_name = unsafe { ffi::gtk_about_dialog_get_logo_icon_name(self.pointer)) };

        if logo_icon_name.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(logo_icon_name) })
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) -> () {
        unsafe {
            logo_icon_name.witch_c_str(|c_str| {
                ffi::gtk_about_dialog_set_logo_icon_name(GTK_ABOUT_DIALOG(self.get_widget()), c_str)
            })
        };
    }

    pub fn add_credit_section(&self, section_name: &str, people: &Vec<String>) -> () {
        unsafe {
            section_name.with_c_str(|c_str| {
                ffi::gtk_about_dialog_add_credit_section(GTK_ABOUT_DIALOG(self.get_widget()), c_str, people.as_slice().as_ptr())
            })
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) -> () {
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl_GtkWidget!(AboutDialog)

impl traits::Widget for Dialog {}
impl traits::Container for Dialog {}
impl traits::Bin for Dialog {}
impl traits::Window for Dialog {}
impl traits::Dialog for Dialog {}