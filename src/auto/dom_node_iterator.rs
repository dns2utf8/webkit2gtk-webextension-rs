// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMNodeIterator(Object<ffi::WebKitDOMNodeIterator>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_node_iterator_get_type(),
    }
}

impl DOMNodeIterator {
    pub fn detach(&self) {
        unsafe {
            ffi::webkit_dom_node_iterator_detach(self.to_glib_none().0);
        }
    }

    pub fn get_expand_entity_references(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_iterator_get_expand_entity_references(self.to_glib_none().0))
        }
    }

    //pub fn get_filter(&self) -> /*Ignored*/Option<DOMNodeFilter> {
    //    unsafe { TODO: call ffi::webkit_dom_node_iterator_get_filter() }
    //}

    pub fn get_pointer_before_reference_node(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_iterator_get_pointer_before_reference_node(self.to_glib_none().0))
        }
    }

    pub fn get_reference_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_iterator_get_reference_node(self.to_glib_none().0))
        }
    }

    pub fn get_root(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_iterator_get_root(self.to_glib_none().0))
        }
    }

    pub fn get_what_to_show(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_node_iterator_get_what_to_show(self.to_glib_none().0)
        }
    }

    pub fn next_node(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_iterator_next_node(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn previous_node(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_iterator_previous_node(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
