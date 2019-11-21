// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct GcString(Interface<aravis_sys::ArvGcString>);

    match fn {
        get_type => || aravis_sys::arv_gc_string_get_type(),
    }
}

pub const NONE_GC_STRING: Option<&GcString> = None;

pub trait GcStringExt: 'static {
    fn get_max_length(&self) -> Result<i64, glib::Error>;

    fn get_value(&self) -> Result<GString, glib::Error>;

    fn set_value(&self, value: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<GcString>> GcStringExt for O {
    fn get_max_length(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_string_get_max_length(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_value(&self) -> Result<GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_gc_string_get_value(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_value(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = aravis_sys::arv_gc_string_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for GcString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcString")
    }
}