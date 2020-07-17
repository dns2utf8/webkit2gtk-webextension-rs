// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
use webkit2_webextension_sys;
use DOMDOMWindow;
use DOMEvent;
use DOMObject;
use DOMUIEvent;

glib_wrapper! {
    pub struct DOMKeyboardEvent(Object<webkit2_webextension_sys::WebKitDOMKeyboardEvent, webkit2_webextension_sys::WebKitDOMKeyboardEventClass, DOMKeyboardEventClass>) @extends DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_keyboard_event_get_type(),
    }
}

pub const NONE_DOM_KEYBOARD_EVENT: Option<&DOMKeyboardEvent> = None;

pub trait DOMKeyboardEventExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_alt_graph_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_alt_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_ctrl_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_key_identifier(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_key_location(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_meta_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_modifier_state(&self, keyIdentifierArg: &str) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_shift_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn init_keyboard_event<P: IsA<DOMDOMWindow>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &P, keyIdentifier: &str, location: libc::c_ulong, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, altGraphKey: bool);

    fn connect_property_alt_graph_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_identifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMKeyboardEvent>> DOMKeyboardEventExt for O {
    fn get_alt_graph_key(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_alt_graph_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_alt_key(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_alt_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_ctrl_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_key_identifier(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_keyboard_event_get_key_identifier(self.as_ref().to_glib_none().0))
        }
    }

    fn get_key_location(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_keyboard_event_get_key_location(self.as_ref().to_glib_none().0)
        }
    }

    fn get_meta_key(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_meta_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_modifier_state(&self, keyIdentifierArg: &str) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_modifier_state(self.as_ref().to_glib_none().0, keyIdentifierArg.to_glib_none().0))
        }
    }

    fn get_shift_key(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_keyboard_event_get_shift_key(self.as_ref().to_glib_none().0))
        }
    }

    fn init_keyboard_event<P: IsA<DOMDOMWindow>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &P, keyIdentifier: &str, location: libc::c_ulong, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, altGraphKey: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_keyboard_event_init_keyboard_event(self.as_ref().to_glib_none().0, type_.to_glib_none().0, canBubble.to_glib(), cancelable.to_glib(), view.as_ref().to_glib_none().0, keyIdentifier.to_glib_none().0, location, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib(), altGraphKey.to_glib());
        }
    }

    fn connect_property_alt_graph_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_graph_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alt-graph-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_alt_graph_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alt-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_alt_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ctrl_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ctrl-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_ctrl_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_key_identifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_identifier_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::key-identifier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_key_identifier_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_key_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_location_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::key-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_key_location_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_meta_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::meta-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_meta_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shift_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMKeyboardEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMKeyboardEvent>
        {
            let f: &F = &*(f as *const F);
            f(&DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shift-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_shift_key_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMKeyboardEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMKeyboardEvent")
    }
}
