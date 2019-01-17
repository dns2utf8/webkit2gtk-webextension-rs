// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ContextMenu;
use ContextMenuAction;
use ffi;
#[cfg(any(feature = "v2_18", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2_18", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ContextMenuItem(Object<ffi::WebKitContextMenuItem, ffi::WebKitContextMenuItemClass, ContextMenuItemClass>);

    match fn {
        get_type => || ffi::webkit_context_menu_item_get_type(),
    }
}

impl ContextMenuItem {
    //#[cfg_attr(feature = "v2_18", deprecated)]
    //pub fn new<P: IsA</*Ignored*/gtk::Action>>(action: &P) -> ContextMenuItem {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_new() }
    //}

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    pub fn new_from_gaction<'a, P: IsA<gio::Action>, Q: Into<Option<&'a glib::Variant>>>(action: &P, label: &str, target: Q) -> ContextMenuItem {
        assert_initialized_main_thread!();
        let target = target.into();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_from_gaction(action.as_ref().to_glib_none().0, label.to_glib_none().0, target.to_glib_none().0))
        }
    }

    pub fn new_from_stock_action(action: ContextMenuAction) -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_from_stock_action(action.to_glib()))
        }
    }

    pub fn new_from_stock_action_with_label(action: ContextMenuAction, label: &str) -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_from_stock_action_with_label(action.to_glib(), label.to_glib_none().0))
        }
    }

    pub fn new_separator() -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_separator())
        }
    }

    pub fn new_with_submenu<P: IsA<ContextMenu>>(label: &str, submenu: &P) -> ContextMenuItem {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_new_with_submenu(label.to_glib_none().0, submenu.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_CONTEXT_MENU_ITEM: Option<&ContextMenuItem> = None;

pub trait ContextMenuItemExt: 'static {
    //#[cfg_attr(feature = "v2_18", deprecated)]
    //fn get_action(&self) -> /*Ignored*/Option<gtk::Action>;

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_gaction(&self) -> Option<gio::Action>;

    fn get_stock_action(&self) -> ContextMenuAction;

    fn get_submenu(&self) -> Option<ContextMenu>;

    fn is_separator(&self) -> bool;

    fn set_submenu<'a, P: IsA<ContextMenu> + 'a, Q: Into<Option<&'a P>>>(&self, submenu: Q);
}

impl<O: IsA<ContextMenuItem>> ContextMenuItemExt for O {
    //fn get_action(&self) -> /*Ignored*/Option<gtk::Action> {
    //    unsafe { TODO: call ffi::webkit_context_menu_item_get_action() }
    //}

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_gaction(&self) -> Option<gio::Action> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_get_gaction(self.as_ref().to_glib_none().0))
        }
    }

    fn get_stock_action(&self) -> ContextMenuAction {
        unsafe {
            from_glib(ffi::webkit_context_menu_item_get_stock_action(self.as_ref().to_glib_none().0))
        }
    }

    fn get_submenu(&self) -> Option<ContextMenu> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_item_get_submenu(self.as_ref().to_glib_none().0))
        }
    }

    fn is_separator(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_context_menu_item_is_separator(self.as_ref().to_glib_none().0))
        }
    }

    fn set_submenu<'a, P: IsA<ContextMenu> + 'a, Q: Into<Option<&'a P>>>(&self, submenu: Q) {
        let submenu = submenu.into();
        unsafe {
            ffi::webkit_context_menu_item_set_submenu(self.as_ref().to_glib_none().0, submenu.map(|p| p.as_ref()).to_glib_none().0);
        }
    }
}

impl fmt::Display for ContextMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContextMenuItem")
    }
}
