// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::ptr;

glib_wrapper! {
    pub struct DOMNamedNodeMap(Object<ffi::WebKitDOMNamedNodeMap>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_named_node_map_get_type(),
    }
}

impl DOMNamedNodeMap {
    pub fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_named_node_map_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    pub fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_item(self.to_glib_none().0, index))
        }
    }

    pub fn remove_named_item(&self, name: &str) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item(self.to_glib_none().0, name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_named_item_ns(&self, namespaceURI: &str, localName: &str) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_named_item<T: IsA<DOMNode>>(&self, node: &T) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item(self.to_glib_none().0, node.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_named_item_ns<T: IsA<DOMNode>>(&self, node: &T) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item_ns(self.to_glib_none().0, node.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
