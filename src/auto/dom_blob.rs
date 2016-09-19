// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMBlob(Object<ffi::WebKitDOMBlob>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_blob_get_type(),
    }
}

pub trait DOMBlobExt {
    fn get_size(&self) -> u64;
}

impl<O: IsA<DOMBlob>> DOMBlobExt for O {
    fn get_size(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_blob_get_size(self.to_glib_none().0)
        }
    }
}