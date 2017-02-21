// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use DOMAttr;
use DOMCSSStyleDeclaration;
use DOMEventTarget;
#[cfg(feature = "v2_10")]
use DOMHTMLCollection;
use DOMNamedNodeMap;
use DOMNode;
use DOMNodeList;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::ptr;

glib_wrapper! {
    pub struct DOMElement(Object<ffi::WebKitDOMElement>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_element_get_type(),
    }
}

pub trait DOMElementExt {
    fn blur(&self);

    fn focus(&self);

    fn get_attribute(&self, name: &str) -> Option<String>;

    fn get_attribute_node(&self, name: &str) -> Option<DOMAttr>;

    fn get_attribute_node_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMAttr>;

    fn get_attribute_ns(&self, namespaceURI: &str, localName: &str) -> Option<String>;

    fn get_attributes(&self) -> Option<DOMNamedNodeMap>;

    fn get_child_element_count(&self) -> libc::c_ulong;

    #[cfg(feature = "v2_10")]
    fn get_children(&self) -> Option<DOMHTMLCollection>;

    fn get_class_name(&self) -> Option<String>;

    fn get_client_height(&self) -> f64;

    fn get_client_left(&self) -> f64;

    fn get_client_top(&self) -> f64;

    fn get_client_width(&self) -> f64;

    fn get_elements_by_class_name(&self, class_name: &str) -> Option<DOMNodeList>;

    #[cfg(feature = "v2_12")]
    fn get_elements_by_class_name_as_html_collection(&self, name: &str) -> Option<DOMHTMLCollection>;

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Option<DOMNodeList>;

    #[cfg(feature = "v2_12")]
    fn get_elements_by_tag_name_as_html_collection(&self, name: &str) -> Option<DOMHTMLCollection>;

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, tag_name: &str) -> Option<DOMNodeList>;

    #[cfg(feature = "v2_12")]
    fn get_elements_by_tag_name_ns_as_html_collection(&self, namespaceURI: &str, localName: &str) -> Option<DOMHTMLCollection>;

    fn get_first_element_child(&self) -> Option<DOMElement>;

    fn get_id(&self) -> Option<String>;

    #[cfg(feature = "v2_8")]
    fn get_inner_html(&self) -> Option<String>;

    fn get_last_element_child(&self) -> Option<DOMElement>;

    fn get_next_element_sibling(&self) -> Option<DOMElement>;

    fn get_offset_height(&self) -> f64;

    fn get_offset_left(&self) -> f64;

    fn get_offset_parent(&self) -> Option<DOMElement>;

    fn get_offset_top(&self) -> f64;

    fn get_offset_width(&self) -> f64;

    #[cfg(feature = "v2_8")]
    fn get_outer_html(&self) -> Option<String>;

    fn get_previous_element_sibling(&self) -> Option<DOMElement>;

    fn get_scroll_height(&self) -> libc::c_long;

    fn get_scroll_left(&self) -> libc::c_long;

    fn get_scroll_top(&self) -> libc::c_long;

    fn get_scroll_width(&self) -> libc::c_long;

    fn get_style(&self) -> Option<DOMCSSStyleDeclaration>;

    fn get_tag_name(&self) -> Option<String>;

    fn has_attribute(&self, name: &str) -> bool;

    fn has_attribute_ns(&self, namespaceURI: &str, localName: &str) -> bool;

    fn has_attributes(&self) -> bool;

    fn query_selector(&self, selectors: &str) -> Result<Option<DOMElement>, Error>;

    fn query_selector_all(&self, selectors: &str) -> Result<DOMNodeList, Error>;

    fn remove_attribute(&self, name: &str);

    fn remove_attribute_node(&self, oldAttr: &DOMAttr) -> Result<DOMAttr, Error>;

    fn remove_attribute_ns(&self, namespaceURI: &str, localName: &str);

    fn scroll_by_lines(&self, lines: libc::c_long);

    fn scroll_by_pages(&self, pages: libc::c_long);

    fn scroll_into_view(&self, alignWithTop: bool);

    fn scroll_into_view_if_needed(&self, centerIfNeeded: bool);

    fn set_attribute(&self, name: &str, value: &str) -> Result<(), Error>;

    fn set_attribute_node(&self, newAttr: &DOMAttr) -> Result<DOMAttr, Error>;

    fn set_attribute_node_ns(&self, newAttr: &DOMAttr) -> Result<DOMAttr, Error>;

    fn set_attribute_ns(&self, namespaceURI: Option<&str>, qualifiedName: &str, value: &str) -> Result<(), Error>;

    fn set_class_name(&self, value: &str);

    fn set_id(&self, value: &str);

    #[cfg(feature = "v2_8")]
    fn set_inner_html(&self, value: &str) -> Result<(), Error>;

    #[cfg(feature = "v2_8")]
    fn set_outer_html(&self, value: &str) -> Result<(), Error>;

    fn set_scroll_left(&self, value: libc::c_long);

    fn set_scroll_top(&self, value: libc::c_long);
}

impl<O: IsA<DOMElement>> DOMElementExt for O {
    fn blur(&self) {
        unsafe {
            ffi::webkit_dom_element_blur(self.to_glib_none().0);
        }
    }

    fn focus(&self) {
        unsafe {
            ffi::webkit_dom_element_focus(self.to_glib_none().0);
        }
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_attribute(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_attribute_node(&self, name: &str) -> Option<DOMAttr> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_attribute_node(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_attribute_node_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMAttr> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_attribute_node_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    fn get_attribute_ns(&self, namespaceURI: &str, localName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_attribute_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    fn get_attributes(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_attributes(self.to_glib_none().0))
        }
    }

    fn get_child_element_count(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_element_get_child_element_count(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_children(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_children(self.to_glib_none().0))
        }
    }

    fn get_class_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_class_name(self.to_glib_none().0))
        }
    }

    fn get_client_height(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_client_height(self.to_glib_none().0)
        }
    }

    fn get_client_left(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_client_left(self.to_glib_none().0)
        }
    }

    fn get_client_top(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_client_top(self.to_glib_none().0)
        }
    }

    fn get_client_width(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_client_width(self.to_glib_none().0)
        }
    }

    fn get_elements_by_class_name(&self, class_name: &str) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_class_name(self.to_glib_none().0, class_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_12")]
    fn get_elements_by_class_name_as_html_collection(&self, name: &str) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_class_name_as_html_collection(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_tag_name(self.to_glib_none().0, tag_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_12")]
    fn get_elements_by_tag_name_as_html_collection(&self, name: &str) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_tag_name_as_html_collection(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, tag_name: &str) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_tag_name_ns(self.to_glib_none().0, namespace_uri.to_glib_none().0, tag_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_12")]
    fn get_elements_by_tag_name_ns_as_html_collection(&self, namespaceURI: &str, localName: &str) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_elements_by_tag_name_ns_as_html_collection(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    fn get_first_element_child(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_first_element_child(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_id(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_8")]
    fn get_inner_html(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_inner_html(self.to_glib_none().0))
        }
    }

    fn get_last_element_child(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_last_element_child(self.to_glib_none().0))
        }
    }

    fn get_next_element_sibling(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_next_element_sibling(self.to_glib_none().0))
        }
    }

    fn get_offset_height(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_offset_height(self.to_glib_none().0)
        }
    }

    fn get_offset_left(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_offset_left(self.to_glib_none().0)
        }
    }

    fn get_offset_parent(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_offset_parent(self.to_glib_none().0))
        }
    }

    fn get_offset_top(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_offset_top(self.to_glib_none().0)
        }
    }

    fn get_offset_width(&self) -> f64 {
        unsafe {
            ffi::webkit_dom_element_get_offset_width(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_8")]
    fn get_outer_html(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_outer_html(self.to_glib_none().0))
        }
    }

    fn get_previous_element_sibling(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_element_get_previous_element_sibling(self.to_glib_none().0))
        }
    }

    fn get_scroll_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_element_get_scroll_height(self.to_glib_none().0)
        }
    }

    fn get_scroll_left(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_element_get_scroll_left(self.to_glib_none().0)
        }
    }

    fn get_scroll_top(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_element_get_scroll_top(self.to_glib_none().0)
        }
    }

    fn get_scroll_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_element_get_scroll_width(self.to_glib_none().0)
        }
    }

    fn get_style(&self) -> Option<DOMCSSStyleDeclaration> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_style(self.to_glib_none().0))
        }
    }

    fn get_tag_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_element_get_tag_name(self.to_glib_none().0))
        }
    }

    fn has_attribute(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_element_has_attribute(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn has_attribute_ns(&self, namespaceURI: &str, localName: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_element_has_attribute_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0))
        }
    }

    fn has_attributes(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_element_has_attributes(self.to_glib_none().0))
        }
    }

    fn query_selector(&self, selectors: &str) -> Result<Option<DOMElement>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_element_query_selector(self.to_glib_none().0, selectors.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn query_selector_all(&self, selectors: &str) -> Result<DOMNodeList, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_element_query_selector_all(self.to_glib_none().0, selectors.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_attribute(&self, name: &str) {
        unsafe {
            ffi::webkit_dom_element_remove_attribute(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn remove_attribute_node(&self, oldAttr: &DOMAttr) -> Result<DOMAttr, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_element_remove_attribute_node(self.to_glib_none().0, oldAttr.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_attribute_ns(&self, namespaceURI: &str, localName: &str) {
        unsafe {
            ffi::webkit_dom_element_remove_attribute_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, localName.to_glib_none().0);
        }
    }

    fn scroll_by_lines(&self, lines: libc::c_long) {
        unsafe {
            ffi::webkit_dom_element_scroll_by_lines(self.to_glib_none().0, lines);
        }
    }

    fn scroll_by_pages(&self, pages: libc::c_long) {
        unsafe {
            ffi::webkit_dom_element_scroll_by_pages(self.to_glib_none().0, pages);
        }
    }

    fn scroll_into_view(&self, alignWithTop: bool) {
        unsafe {
            ffi::webkit_dom_element_scroll_into_view(self.to_glib_none().0, alignWithTop.to_glib());
        }
    }

    fn scroll_into_view_if_needed(&self, centerIfNeeded: bool) {
        unsafe {
            ffi::webkit_dom_element_scroll_into_view_if_needed(self.to_glib_none().0, centerIfNeeded.to_glib());
        }
    }

    fn set_attribute(&self, name: &str, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_element_set_attribute(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_attribute_node(&self, newAttr: &DOMAttr) -> Result<DOMAttr, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_element_set_attribute_node(self.to_glib_none().0, newAttr.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_attribute_node_ns(&self, newAttr: &DOMAttr) -> Result<DOMAttr, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_element_set_attribute_node_ns(self.to_glib_none().0, newAttr.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_attribute_ns(&self, namespaceURI: Option<&str>, qualifiedName: &str, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_element_set_attribute_ns(self.to_glib_none().0, namespaceURI.to_glib_none().0, qualifiedName.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_class_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_element_set_class_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_id(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_element_set_id(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    fn set_inner_html(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_element_set_inner_html(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_8")]
    fn set_outer_html(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_element_set_outer_html(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_scroll_left(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_element_set_scroll_left(self.to_glib_none().0, value);
        }
    }

    fn set_scroll_top(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_element_set_scroll_top(self.to_glib_none().0, value);
        }
    }
}
