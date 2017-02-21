// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLModElement(Object<ffi::WebKitDOMHTMLModElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_mod_element_get_type(),
    }
}

impl DOMHTMLModElement {
    pub fn get_cite(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_mod_element_get_cite(self.to_glib_none().0))
        }
    }

    pub fn get_date_time(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_mod_element_get_date_time(self.to_glib_none().0))
        }
    }

    pub fn set_cite(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_mod_element_set_cite(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_date_time(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_mod_element_set_date_time(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
