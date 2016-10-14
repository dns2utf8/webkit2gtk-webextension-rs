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
    pub struct DOMHTMLBaseFontElement(Object<ffi::WebKitDOMHTMLBaseFontElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_base_font_element_get_type(),
    }
}

impl DOMHTMLBaseFontElement {
    pub fn get_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_font_element_get_color(self.to_glib_none().0))
        }
    }

    pub fn get_face(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_font_element_get_face(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_base_font_element_get_size(self.to_glib_none().0)
        }
    }

    pub fn set_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_face(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_face(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_size(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_base_font_element_set_size(self.to_glib_none().0, value);
        }
    }
}
