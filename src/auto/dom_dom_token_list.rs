// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMObject;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use Error;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMDOMTokenList(Object<webkit2_webextension_sys::WebKitDOMDOMTokenList, webkit2_webextension_sys::WebKitDOMDOMTokenListClass, DOMDOMTokenListClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_dom_token_list_get_type(),
    }
}

pub const NONE_DOMDOM_TOKEN_LIST: Option<&DOMDOMTokenList> = None;

pub trait DOMDOMTokenListExt: 'static {
    //#[cfg_attr(feature = "v2_22", deprecated)]
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn add(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn contains(&self, token: &str) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_value(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<GString>;

    //#[cfg_attr(feature = "v2_22", deprecated)]
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn replace(&self, token: &str, newToken: &str) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_value(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn toggle(&self, token: &str, force: bool) -> Result<(), Error>;

    fn get_property_length(&self) -> libc::c_ulong;

    fn get_property_value(&self) -> Option<GString>;

    fn set_property_value(&self, value: Option<&str>);

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDOMTokenList>> DOMDOMTokenListExt for O {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn add(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call webkit2_webextension_sys:webkit_dom_dom_token_list_add() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn contains(&self, token: &str) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_dom_token_list_contains(self.as_ref().to_glib_none().0, token.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_dom_token_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_dom_token_list_get_value(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_dom_token_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call webkit2_webextension_sys:webkit_dom_dom_token_list_remove() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn replace(&self, token: &str, newToken: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_dom_token_list_replace(self.as_ref().to_glib_none().0, token.to_glib_none().0, newToken.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_value(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_dom_token_list_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn toggle(&self, token: &str, force: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_dom_token_list_toggle(self.as_ref().to_glib_none().0, token.to_glib_none().0, force.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_length(&self) -> libc::c_ulong {
        unsafe {
            let mut value = Value::from_type(<libc::c_ulong as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"length\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_value(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"value\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_value(&self, value: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"value\0".as_ptr() as *const _, Value::from(value).to_glib_none().0);
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDOMTokenList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMDOMTokenList>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMTokenList::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMDOMTokenList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMDOMTokenList>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMTokenList::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMDOMTokenList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMDOMTokenList")
    }
}
