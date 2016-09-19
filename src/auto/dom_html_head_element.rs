// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLHeadElement(Object<ffi::WebKitDOMHTMLHeadElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_head_element_get_type(),
    }
}

impl DOMHTMLHeadElement {
    pub fn get_profile(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_head_element_get_profile(self.to_glib_none().0))
        }
    }

    pub fn set_profile(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_head_element_set_profile(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}