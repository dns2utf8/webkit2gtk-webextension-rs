// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLFieldSetElement(Object<webkit2_webextension_sys::WebKitDOMHTMLFieldSetElement, webkit2_webextension_sys::WebKitDOMHTMLFieldSetElementClass, DOMHTMLFieldSetElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_field_set_element_get_type(),
    }
}

pub const NONE_DOMHTML_FIELD_SET_ELEMENT: Option<&DOMHTMLFieldSetElement> = None;

pub trait DOMHTMLFieldSetElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFieldSetElement>> DOMHTMLFieldSetElementExt for O {
    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_field_set_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_form_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLFieldSetElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLFieldSetElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFieldSetElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_form_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLFieldSetElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFieldSetElement")
    }
}
