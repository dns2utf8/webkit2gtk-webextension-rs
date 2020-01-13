// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLPreElement(Object<webkit2_webextension_sys::WebKitDOMHTMLPreElement, webkit2_webextension_sys::WebKitDOMHTMLPreElementClass, DOMHTMLPreElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_pre_element_get_type(),
    }
}

pub const NONE_DOMHTML_PRE_ELEMENT: Option<&DOMHTMLPreElement> = None;

pub trait DOMHTMLPreElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_wrap(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_wrap(&self, value: bool);

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLPreElement>> DOMHTMLPreElementExt for O {
    fn get_width(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_pre_element_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_pre_element_get_wrap(self.as_ref().to_glib_none().0))
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_pre_element_set_width(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_pre_element_set_wrap(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLPreElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLPreElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLPreElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute(notify_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLPreElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLPreElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLPreElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLPreElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLPreElement")
    }
}
