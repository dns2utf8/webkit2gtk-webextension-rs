// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLFrameSetElement(Object<ffi::WebKitDOMHTMLFrameSetElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_frame_set_element_get_type(),
    }
}

impl DOMHTMLFrameSetElement {
    pub fn get_cols(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_cols(self.to_glib_none().0))
        }
    }

    pub fn get_rows(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_rows(self.to_glib_none().0))
        }
    }

    pub fn set_cols(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_cols(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_rows(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_rows(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
