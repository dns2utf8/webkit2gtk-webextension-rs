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
    pub struct DOMHTMLParamElement(Object<ffi::WebKitDOMHTMLParamElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_param_element_get_type(),
    }
}

impl DOMHTMLParamElement {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_type_attr(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_value(self.to_glib_none().0))
        }
    }

    pub fn get_value_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_param_element_get_value_type(self.to_glib_none().0))
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_value_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_param_element_set_value_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
