// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMCSSStyleSheet;
use DOMDocument;
use DOMDocumentType;
use DOMHTMLDocument;
use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMDOMImplementation(Object<ffi::WebKitDOMDOMImplementation>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_implementation_get_type(),
    }
}

impl DOMDOMImplementation {
    pub fn create_css_style_sheet(&self, title: &str, media: &str) -> Result<DOMCSSStyleSheet, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_css_style_sheet(self.to_glib_none().0, title.to_glib_none().0, media.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn create_document(&self, namespaceURI: Option<&str>, qualifiedName: &str, doctype: Option<&DOMDocumentType>) -> Result<DOMDocument, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document(self.to_glib_none().0, namespaceURI.to_glib_none().0, qualifiedName.to_glib_none().0, doctype.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn create_document_type(&self, qualifiedName: &str, publicId: &str, systemId: &str) -> Result<DOMDocumentType, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document_type(self.to_glib_none().0, qualifiedName.to_glib_none().0, publicId.to_glib_none().0, systemId.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn create_html_document(&self, title: &str) -> Option<DOMHTMLDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_implementation_create_html_document(self.to_glib_none().0, title.to_glib_none().0))
        }
    }

    pub fn has_feature(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_implementation_has_feature(self.to_glib_none().0, feature.to_glib_none().0, version.to_glib_none().0))
        }
    }
}
