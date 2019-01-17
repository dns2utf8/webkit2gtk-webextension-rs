// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLOListElement(Object<ffi::WebKitDOMHTMLOListElement, ffi::WebKitDOMHTMLOListElementClass, DOMHTMLOListElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_o_list_element_get_type(),
    }
}

pub const NONE_DOMHTMLO_LIST_ELEMENT: Option<&DOMHTMLOListElement> = None;

pub trait DOMHTMLOListElementExt: 'static {
    fn get_compact(&self) -> bool;

    fn get_start(&self) -> libc::c_long;

    fn get_type_attr(&self) -> Option<GString>;

    fn set_compact(&self, value: bool);

    fn set_start(&self, value: libc::c_long);

    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOListElement>> DOMHTMLOListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_o_list_element_get_compact(self.as_ref().to_glib_none().0))
        }
    }

    fn get_start(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_o_list_element_get_start(self.as_ref().to_glib_none().0)
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_o_list_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_compact(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_start(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_start(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P) {
        let type_ = type_.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::compact\0".as_ptr() as *const _,
                transmute(notify_compact_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::start\0".as_ptr() as *const _,
                transmute(notify_start_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_compact_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_start_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLOListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLOListElement")
    }
}
