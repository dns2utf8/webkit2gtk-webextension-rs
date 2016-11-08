// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMEventTarget;
use DOMNamedNodeMap;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMDocumentType(Object<ffi::WebKitDOMDocumentType>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_document_type_get_type(),
    }
}

impl DOMDocumentType {
    pub fn get_entities(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_entities(self.to_glib_none().0))
        }
    }

    pub fn get_internal_subset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_internal_subset(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_notations(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_notations(self.to_glib_none().0))
        }
    }

    pub fn get_public_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_public_id(self.to_glib_none().0))
        }
    }

    pub fn get_system_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_system_id(self.to_glib_none().0))
        }
    }
}
