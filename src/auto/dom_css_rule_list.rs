// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMCSSRule;
use DOMObject;
use ffi;
use glib::translate::*;
use libc;

glib_wrapper! {
    pub struct DOMCSSRuleList(Object<ffi::WebKitDOMCSSRuleList>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_rule_list_get_type(),
    }
}

impl DOMCSSRuleList {
    pub fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_css_rule_list_get_length(self.to_glib_none().0)
        }
    }

    pub fn item(&self, index: libc::c_ulong) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_rule_list_item(self.to_glib_none().0, index))
        }
    }
}
