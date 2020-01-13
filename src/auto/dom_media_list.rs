// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use webkit2_webextension_sys;
use DOMObject;

glib_wrapper! {
    pub struct DOMMediaList(Object<webkit2_webextension_sys::WebKitDOMMediaList, webkit2_webextension_sys::WebKitDOMMediaListClass, DOMMediaListClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_media_list_get_type(),
    }
}

pub const NONE_DOM_MEDIA_LIST: Option<&DOMMediaList> = None;

pub trait DOMMediaListExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn append_medium(&self, newMedium: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_medium(&self, oldMedium: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_media_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_media_text(&self, value: &str) -> Result<(), glib::Error>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_media_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMMediaList>> DOMMediaListExt for O {
    fn append_medium(&self, newMedium: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_media_list_append_medium(self.as_ref().to_glib_none().0, newMedium.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_medium(&self, oldMedium: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_media_list_delete_medium(self.as_ref().to_glib_none().0, oldMedium.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_media_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn get_media_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_media_list_get_media_text(self.as_ref().to_glib_none().0))
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_media_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn set_media_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_media_list_set_media_text(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMMediaList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMMediaList>
        {
            let f: &F = &*(f as *const F);
            f(&DOMMediaList::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_media_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMMediaList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMMediaList>
        {
            let f: &F = &*(f as *const F);
            f(&DOMMediaList::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::media-text\0".as_ptr() as *const _,
                Some(transmute(notify_media_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMMediaList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMMediaList")
    }
}
