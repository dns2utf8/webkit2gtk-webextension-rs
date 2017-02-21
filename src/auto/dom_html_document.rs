// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMDocument;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;
use libc;

glib_wrapper! {
    pub struct DOMHTMLDocument(Object<ffi::WebKitDOMHTMLDocument>): DOMDocument, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_document_get_type(),
    }
}

impl DOMHTMLDocument {
    pub fn capture_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_capture_events(self.to_glib_none().0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::webkit_dom_html_document_clear(self.to_glib_none().0);
        }
    }

    pub fn close(&self) {
        unsafe {
            ffi::webkit_dom_html_document_close(self.to_glib_none().0);
        }
    }

    pub fn get_alink_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_alink_color(self.to_glib_none().0))
        }
    }

    pub fn get_bg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_bg_color(self.to_glib_none().0))
        }
    }

    pub fn get_compat_mode(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_compat_mode(self.to_glib_none().0))
        }
    }

    pub fn get_design_mode(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_design_mode(self.to_glib_none().0))
        }
    }

    pub fn get_dir(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_dir(self.to_glib_none().0))
        }
    }

    pub fn get_embeds(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_embeds(self.to_glib_none().0))
        }
    }

    pub fn get_fg_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_fg_color(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_document_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_link_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_link_color(self.to_glib_none().0))
        }
    }

    pub fn get_plugins(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_plugins(self.to_glib_none().0))
        }
    }

    pub fn get_scripts(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_scripts(self.to_glib_none().0))
        }
    }

    pub fn get_vlink_color(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_document_get_vlink_color(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_document_get_width(self.to_glib_none().0)
        }
    }

    pub fn release_events(&self) {
        unsafe {
            ffi::webkit_dom_html_document_release_events(self.to_glib_none().0);
        }
    }

    pub fn set_alink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_alink_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_bg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_bg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_design_mode(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_design_mode(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_dir(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_dir(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_fg_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_fg_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_link_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_link_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_vlink_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_document_set_vlink_color(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
