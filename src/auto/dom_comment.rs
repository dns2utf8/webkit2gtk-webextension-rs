// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMComment(Object<ffi::WebKitDOMComment>): DOMCharacterData, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_comment_get_type(),
    }
}

impl DOMComment {}
