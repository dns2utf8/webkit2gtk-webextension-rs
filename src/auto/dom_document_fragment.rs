// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMEventTarget;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMDocumentFragment(Object<ffi::WebKitDOMDocumentFragment>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_document_fragment_get_type(),
    }
}

impl DOMDocumentFragment {}
