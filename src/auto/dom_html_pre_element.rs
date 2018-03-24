// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLPreElement(Object<ffi::WebKitDOMHTMLPreElement, ffi::WebKitDOMHTMLPreElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_pre_element_get_type(),
    }
}

pub trait DOMHTMLPreElementExt {
    fn get_width(&self) -> libc::c_long;

    fn get_wrap(&self) -> bool;

    fn set_width(&self, value: libc::c_long);

    fn set_wrap(&self, value: bool);

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLPreElement> + IsA<glib::object::Object>> DOMHTMLPreElementExt for O {
    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_pre_element_get_width(self.to_glib_none().0)
        }
    }

    fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_pre_element_get_wrap(self.to_glib_none().0))
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_pre_element_set_width(self.to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_pre_element_set_wrap(self.to_glib_none().0, value.to_glib());
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::width",
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wrap",
                transmute(notify_wrap_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLPreElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLPreElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLPreElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wrap_trampoline<P>(this: *mut ffi::WebKitDOMHTMLPreElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLPreElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLPreElement::from_glib_borrow(this).downcast_unchecked())
}
