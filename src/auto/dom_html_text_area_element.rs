// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;
use libc;

glib_wrapper! {
    pub struct DOMHTMLTextAreaElement(Object<ffi::WebKitDOMHTMLTextAreaElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_text_area_element_get_type(),
    }
}

impl DOMHTMLTextAreaElement {
    pub fn get_area_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_area_type(self.to_glib_none().0))
        }
    }

    pub fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_autofocus(self.to_glib_none().0))
        }
    }

    pub fn get_cols(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_cols(self.to_glib_none().0)
        }
    }

    pub fn get_default_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_default_value(self.to_glib_none().0))
        }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_disabled(self.to_glib_none().0))
        }
    }

    pub fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_text_area_element_get_form(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_read_only(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_read_only(self.to_glib_none().0))
        }
    }

    pub fn get_rows(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_rows(self.to_glib_none().0)
        }
    }

    pub fn get_selection_end(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_selection_end(self.to_glib_none().0)
        }
    }

    pub fn get_selection_start(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_selection_start(self.to_glib_none().0)
        }
    }

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_value(self.to_glib_none().0))
        }
    }

    pub fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_will_validate(self.to_glib_none().0))
        }
    }

    pub fn is_edited(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_is_edited(self.to_glib_none().0))
        }
    }

    pub fn select(&self) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_select(self.to_glib_none().0);
        }
    }

    pub fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_cols(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_cols(self.to_glib_none().0, value);
        }
    }

    pub fn set_default_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_default_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_read_only(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_read_only(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_rows(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_rows(self.to_glib_none().0, value);
        }
    }

    pub fn set_selection_end(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_end(self.to_glib_none().0, value);
        }
    }

    pub fn set_selection_range(&self, start: libc::c_long, end: libc::c_long, direction: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_range(self.to_glib_none().0, start, end, direction.to_glib_none().0);
        }
    }

    pub fn set_selection_start(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_start(self.to_glib_none().0, value);
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
