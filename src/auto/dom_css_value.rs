// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSValue(Object<ffi::WebKitDOMCSSValue>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_value_get_type(),
    }
}

impl DOMCSSValue {
    pub fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_value_get_css_text(self.to_glib_none().0))
        }
    }

    pub fn get_css_value_type(&self) -> u16 {
        unsafe {
            ffi::webkit_dom_css_value_get_css_value_type(self.to_glib_none().0)
        }
    }

    pub fn set_css_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_value_set_css_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
