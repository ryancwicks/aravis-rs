// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DomElement;
use DomNode;
use GcFeatureNode;
use GcFloat;
use GcNode;
use aravis_sys;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct GcFloatNode(Object<aravis_sys::ArvGcFloatNode, aravis_sys::ArvGcFloatNodeClass, GcFloatNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode, @implements GcFloat;

    match fn {
        get_type => || aravis_sys::arv_gc_float_node_get_type(),
    }
}

impl GcFloatNode {
    pub fn new() -> GcFloatNode {
        assert_initialized_main_thread!();
        unsafe {
            GcNode::from_glib_full(aravis_sys::arv_gc_float_node_new()).unsafe_cast()
        }
    }
}

impl Default for GcFloatNode {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GC_FLOAT_NODE: Option<&GcFloatNode> = None;

impl fmt::Display for GcFloatNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GcFloatNode")
    }
}