// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMObject;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMBlob(Object<ffi::WebKitDOMBlob, ffi::WebKitDOMBlobClass, DOMBlobClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_blob_get_type(),
    }
}

pub const NONE_DOM_BLOB: Option<&DOMBlob> = None;

pub trait DOMBlobExt: 'static {
    fn get_size(&self) -> u64;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMBlob>> DOMBlobExt for O {
    fn get_size(&self) -> u64 {
        unsafe {
            ffi::webkit_dom_blob_get_size(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::WebKitDOMBlob, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMBlob> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMBlob::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMBlob")
    }
}
