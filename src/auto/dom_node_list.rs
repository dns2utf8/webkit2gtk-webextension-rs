// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMNodeList(Object<ffi::WebKitDOMNodeList, ffi::WebKitDOMNodeListClass, DOMNodeListClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_node_list_get_type(),
    }
}

pub const NONE_DOM_NODE_LIST: Option<&DOMNodeList> = None;

pub trait DOMNodeListExt: 'static {
    fn get_length(&self) -> libc::c_ulong;

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNodeList>> DOMNodeListExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_node_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMNodeList, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMNodeList> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMNodeList::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMNodeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMNodeList")
    }
}
