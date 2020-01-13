// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
#[cfg(any(feature = "v2_18", feature = "dox"))]
use DOMClientRect;
use DOMObject;

glib_wrapper! {
    pub struct DOMClientRectList(Object<webkit2_webextension_sys::WebKitDOMClientRectList, webkit2_webextension_sys::WebKitDOMClientRectListClass, DOMClientRectListClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_client_rect_list_get_type(),
    }
}

pub const NONE_DOM_CLIENT_RECT_LIST: Option<&DOMClientRectList> = None;

pub trait DOMClientRectListExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<DOMClientRect>;

    fn get_property_length(&self) -> libc::c_ulong;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMClientRectList>> DOMClientRectListExt for O {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_client_rect_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<DOMClientRect> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_client_rect_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_property_length(&self) -> libc::c_ulong {
        unsafe {
            let mut value = Value::from_type(<libc::c_ulong as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"length\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `length` getter").unwrap()
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMClientRectList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMClientRectList>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRectList::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMClientRectList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMClientRectList")
    }
}
