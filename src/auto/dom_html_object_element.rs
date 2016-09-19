// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMDocument;
use DOMElement;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLObjectElement(Object<ffi::WebKitDOMHTMLObjectElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_object_element_get_type(),
    }
}

impl DOMHTMLObjectElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_archive(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_archive(self.to_glib_none().0))
        }
    }

    pub fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_border(self.to_glib_none().0))
        }
    }

    pub fn get_code(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code(self.to_glib_none().0))
        }
    }

    pub fn get_code_base(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_base(self.to_glib_none().0))
        }
    }

    pub fn get_code_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_type(self.to_glib_none().0))
        }
    }

    pub fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_content_document(self.to_glib_none().0))
        }
    }

    pub fn get_data(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_data(self.to_glib_none().0))
        }
    }

    pub fn get_declare(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_object_element_get_declare(self.to_glib_none().0))
        }
    }

    pub fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_form(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_height(self.to_glib_none().0))
        }
    }

    //pub fn get_hspace(&self) -> /*Unimplemented*/Fundamental: Long {
    //    unsafe { TODO: call ffi::webkit_dom_html_object_element_get_hspace() }
    //}

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_standby(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_standby(self.to_glib_none().0))
        }
    }

    pub fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_type_attr(self.to_glib_none().0))
        }
    }

    pub fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_use_map(self.to_glib_none().0))
        }
    }

    //pub fn get_vspace(&self) -> /*Unimplemented*/Fundamental: Long {
    //    unsafe { TODO: call ffi::webkit_dom_html_object_element_get_vspace() }
    //}

    pub fn get_width(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_width(self.to_glib_none().0))
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_archive(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_archive(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_code(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_code_base(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_base(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_code_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_data(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_data(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_declare(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_declare(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_height(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    //pub fn set_hspace(&self, value: /*Unimplemented*/Fundamental: Long) {
    //    unsafe { TODO: call ffi::webkit_dom_html_object_element_set_hspace() }
    //}

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_standby(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_standby(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    //pub fn set_vspace(&self, value: /*Unimplemented*/Fundamental: Long) {
    //    unsafe { TODO: call ffi::webkit_dom_html_object_element_set_vspace() }
    //}

    pub fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_width(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}