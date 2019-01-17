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
    pub struct DOMHTMLMetaElement(Object<ffi::WebKitDOMHTMLMetaElement, ffi::WebKitDOMHTMLMetaElementClass, DOMHTMLMetaElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_meta_element_get_type(),
    }
}

pub const NONE_DOMHTML_META_ELEMENT: Option<&DOMHTMLMetaElement> = None;

pub trait DOMHTMLMetaElementExt: 'static {
    fn get_content(&self) -> Option<GString>;

    fn get_http_equiv(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_scheme(&self) -> Option<GString>;

    fn set_content(&self, value: &str);

    fn set_http_equiv(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_scheme(&self, value: &str);

    fn connect_property_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_http_equiv_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLMetaElement>> DOMHTMLMetaElementExt for O {
    fn get_content(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_content(self.as_ref().to_glib_none().0))
        }
    }

    fn get_http_equiv(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_http_equiv(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_scheme(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_meta_element_get_scheme(self.as_ref().to_glib_none().0))
        }
    }

    fn set_content(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_content(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_http_equiv(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_http_equiv(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_scheme(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_meta_element_set_scheme(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::content\0".as_ptr() as *const _,
                transmute(notify_content_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_http_equiv_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::http-equiv\0".as_ptr() as *const _,
                transmute(notify_http_equiv_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::scheme\0".as_ptr() as *const _,
                transmute(notify_scheme_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_content_trampoline<P>(this: *mut ffi::WebKitDOMHTMLMetaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLMetaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLMetaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_http_equiv_trampoline<P>(this: *mut ffi::WebKitDOMHTMLMetaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLMetaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLMetaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLMetaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLMetaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLMetaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_scheme_trampoline<P>(this: *mut ffi::WebKitDOMHTMLMetaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLMetaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLMetaElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLMetaElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLMetaElement")
    }
}
