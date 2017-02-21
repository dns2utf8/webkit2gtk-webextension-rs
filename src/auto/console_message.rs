// This file was generated by gir (1ec7e91) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct ConsoleMessage(Boxed<ffi::WebKitConsoleMessage>);

    match fn {
        copy => |ptr| ffi::webkit_console_message_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_console_message_free(ptr),
    }
}

impl ConsoleMessage {
    //#[cfg(feature = "v2_12")]
    //pub fn get_level(&mut self) -> /*Ignored*/ConsoleMessageLevel {
    //    unsafe { TODO: call ffi::webkit_console_message_get_level() }
    //}

    #[cfg(feature = "v2_12")]
    pub fn get_line(&mut self) -> u32 {
        unsafe {
            ffi::webkit_console_message_get_line(self.to_glib_none_mut().0)
        }
    }

    //#[cfg(feature = "v2_12")]
    //pub fn get_source(&mut self) -> /*Ignored*/ConsoleMessageSource {
    //    unsafe { TODO: call ffi::webkit_console_message_get_source() }
    //}

    #[cfg(feature = "v2_12")]
    pub fn get_source_id(&mut self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_console_message_get_source_id(self.to_glib_none_mut().0))
        }
    }

    #[cfg(feature = "v2_12")]
    pub fn get_text(&mut self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_console_message_get_text(self.to_glib_none_mut().0))
        }
    }
}
