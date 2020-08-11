use x11::xlib;
use std::ptr;
use std::ffi::{CString, NulError};
use std::os::raw::c_char;

pub struct XWindow {
    display: *mut xlib::Display,
    id_window: xlib::Window,
}

impl XWindow {
    pub fn init() -> Result<Self, &'static str> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());

            if display.is_null() {
                return Err("cannot open display");
            }

            let screen = xlib::XDefaultScreen(display);
            let id_window = xlib::XRootWindow(display, screen);

            Ok(Self {
                display,
                id_window,
            })
        }
    }

    pub fn render(&self, text: String) -> Result<(), NulError> {
        let status_c = CString::new(text).unwrap();

        unsafe {
            xlib::XStoreName(
                self.display,
                self.id_window,
                status_c.as_ptr() as *mut c_char,
            );

            xlib::XFlush(self.display);
        }

        Ok(())
    }
}

impl Drop for XWindow {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}