// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use libc;
use std::ptr;

glib_wrapper! {
    pub struct DOMMediaList(Object<ffi::WebKitDOMMediaList>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_media_list_get_type(),
    }
}

impl DOMMediaList {
    pub fn append_medium(&self, newMedium: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_media_list_append_medium(self.to_glib_none().0, newMedium.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn delete_medium(&self, oldMedium: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_media_list_delete_medium(self.to_glib_none().0, oldMedium.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_media_list_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_media_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_media_list_get_media_text(self.to_glib_none().0))
        }
    }

    pub fn item(&self, index: libc::c_ulong) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_media_list_item(self.to_glib_none().0, index))
        }
    }

    pub fn set_media_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_media_list_set_media_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
