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
    pub struct Evaluator(Object<aravis_sys::ArvEvaluator, aravis_sys::ArvEvaluatorClass, EvaluatorClass>);

    match fn {
        get_type => || aravis_sys::arv_evaluator_get_type(),
    }
}

impl Evaluator {
    pub fn new(expression: Option<&str>) -> Evaluator {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(aravis_sys::arv_evaluator_new(expression.to_glib_none().0))
        }
    }
}

pub const NONE_EVALUATOR: Option<&Evaluator> = None;

pub trait EvaluatorExt: 'static {
    fn evaluate_as_double(&self) -> Result<f64, glib::Error>;

    fn evaluate_as_int64(&self) -> Result<i64, glib::Error>;

    fn get_constant(&self, name: &str) -> Option<GString>;

    fn get_expression(&self) -> Option<GString>;

    fn get_sub_expression(&self, name: &str) -> Option<GString>;

    fn set_constant(&self, name: &str, constant: Option<&str>);

    fn set_double_variable(&self, name: &str, v_double: f64);

    fn set_expression(&self, expression: &str);

    fn set_int64_variable(&self, name: &str, v_int64: i64);

    fn set_sub_expression(&self, name: &str, expression: Option<&str>);
}

impl<O: IsA<Evaluator>> EvaluatorExt for O {
    fn evaluate_as_double(&self) -> Result<f64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_evaluator_evaluate_as_double(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn evaluate_as_int64(&self) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = aravis_sys::arv_evaluator_evaluate_as_int64(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_constant(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_evaluator_get_constant(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_expression(&self) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_evaluator_get_expression(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sub_expression(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(aravis_sys::arv_evaluator_get_sub_expression(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn set_constant(&self, name: &str, constant: Option<&str>) {
        unsafe {
            aravis_sys::arv_evaluator_set_constant(self.as_ref().to_glib_none().0, name.to_glib_none().0, constant.to_glib_none().0);
        }
    }

    fn set_double_variable(&self, name: &str, v_double: f64) {
        unsafe {
            aravis_sys::arv_evaluator_set_double_variable(self.as_ref().to_glib_none().0, name.to_glib_none().0, v_double);
        }
    }

    fn set_expression(&self, expression: &str) {
        unsafe {
            aravis_sys::arv_evaluator_set_expression(self.as_ref().to_glib_none().0, expression.to_glib_none().0);
        }
    }

    fn set_int64_variable(&self, name: &str, v_int64: i64) {
        unsafe {
            aravis_sys::arv_evaluator_set_int64_variable(self.as_ref().to_glib_none().0, name.to_glib_none().0, v_int64);
        }
    }

    fn set_sub_expression(&self, name: &str, expression: Option<&str>) {
        unsafe {
            aravis_sys::arv_evaluator_set_sub_expression(self.as_ref().to_glib_none().0, name.to_glib_none().0, expression.to_glib_none().0);
        }
    }
}

impl fmt::Display for Evaluator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluator")
    }
}