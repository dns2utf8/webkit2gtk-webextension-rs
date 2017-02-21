// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMDOMWindow;
use DOMEvent;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use DOMUIEvent;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use libc;

glib_wrapper! {
    pub struct DOMMouseEvent(Object<ffi::WebKitDOMMouseEvent>): DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_mouse_event_get_type(),
    }
}

pub trait DOMMouseEventExt {
    fn get_alt_key(&self) -> bool;

    fn get_button(&self) -> libc::c_ushort;

    fn get_client_x(&self) -> libc::c_long;

    fn get_client_y(&self) -> libc::c_long;

    fn get_ctrl_key(&self) -> bool;

    fn get_from_element(&self) -> Option<DOMNode>;

    fn get_meta_key(&self) -> bool;

    fn get_offset_x(&self) -> libc::c_long;

    fn get_offset_y(&self) -> libc::c_long;

    fn get_related_target(&self) -> Option<DOMEventTarget>;

    fn get_screen_x(&self) -> libc::c_long;

    fn get_screen_y(&self) -> libc::c_long;

    fn get_shift_key(&self) -> bool;

    fn get_to_element(&self) -> Option<DOMNode>;

    fn get_x(&self) -> libc::c_long;

    fn get_y(&self) -> libc::c_long;

    fn init_mouse_event<T: IsA<DOMEventTarget>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, detail: libc::c_long, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, button: libc::c_ushort, relatedTarget: &T);
}

impl<O: IsA<DOMMouseEvent>> DOMMouseEventExt for O {
    fn get_alt_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_alt_key(self.to_glib_none().0))
        }
    }

    fn get_button(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_mouse_event_get_button(self.to_glib_none().0)
        }
    }

    fn get_client_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_client_x(self.to_glib_none().0)
        }
    }

    fn get_client_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_client_y(self.to_glib_none().0)
        }
    }

    fn get_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_ctrl_key(self.to_glib_none().0))
        }
    }

    fn get_from_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_from_element(self.to_glib_none().0))
        }
    }

    fn get_meta_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_meta_key(self.to_glib_none().0))
        }
    }

    fn get_offset_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_offset_x(self.to_glib_none().0)
        }
    }

    fn get_offset_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_offset_y(self.to_glib_none().0)
        }
    }

    fn get_related_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_mouse_event_get_related_target(self.to_glib_none().0))
        }
    }

    fn get_screen_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_screen_x(self.to_glib_none().0)
        }
    }

    fn get_screen_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_screen_y(self.to_glib_none().0)
        }
    }

    fn get_shift_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_shift_key(self.to_glib_none().0))
        }
    }

    fn get_to_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_to_element(self.to_glib_none().0))
        }
    }

    fn get_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_x(self.to_glib_none().0)
        }
    }

    fn get_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_mouse_event_get_y(self.to_glib_none().0)
        }
    }

    fn init_mouse_event<T: IsA<DOMEventTarget>>(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, detail: libc::c_long, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, button: libc::c_ushort, relatedTarget: &T) {
        unsafe {
            ffi::webkit_dom_mouse_event_init_mouse_event(self.to_glib_none().0, type_.to_glib_none().0, canBubble.to_glib(), cancelable.to_glib(), view.to_glib_none().0, detail, screenX, screenY, clientX, clientY, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib(), button, relatedTarget.to_glib_none().0);
        }
    }
}
