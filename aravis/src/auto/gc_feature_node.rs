// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::DomElement;
use crate::DomNode;
use crate::GcAccessMode;
use crate::GcNameSpace;
use crate::GcNode;
use crate::GcVisibility;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
	#[doc(alias = "ArvGcFeatureNode")]
	pub struct GcFeatureNode(Object<ffi::ArvGcFeatureNode, ffi::ArvGcFeatureNodeClass>) @extends GcNode, DomElement, DomNode;

	match fn {
		type_ => || ffi::arv_gc_feature_node_get_type(),
	}
}

unsafe impl Send for GcFeatureNode {}

pub const NONE_GC_FEATURE_NODE: Option<&GcFeatureNode> = None;

/// Trait containing all [`struct@GcFeatureNode`] methods.
///
/// # Implementors
///
/// [`GcBoolean`][struct@crate::GcBoolean], [`GcCategory`][struct@crate::GcCategory], [`GcCommand`][struct@crate::GcCommand], [`GcConverter`][struct@crate::GcConverter], [`GcEnumEntry`][struct@crate::GcEnumEntry], [`GcEnumeration`][struct@crate::GcEnumeration], [`GcFeatureNode`][struct@crate::GcFeatureNode], [`GcFloatNode`][struct@crate::GcFloatNode], [`GcGroupNode`][struct@crate::GcGroupNode], [`GcIntegerNode`][struct@crate::GcIntegerNode], [`GcPort`][struct@crate::GcPort], [`GcRegisterDescriptionNode`][struct@crate::GcRegisterDescriptionNode], [`GcRegisterNode`][struct@crate::GcRegisterNode], [`GcStructEntryNode`][struct@crate::GcStructEntryNode], [`GcSwissKnife`][struct@crate::GcSwissKnife]
pub trait GcFeatureNodeExt: 'static {
	/// Gets feature node allowed access mode. This is a combination of Genicam ImposedAccessMode and
	/// AccessMode properties of underlying features and registers.
	///
	/// # Returns
	///
	/// Access mode as [`GcAccessMode`][crate::GcAccessMode]
	#[doc(alias = "arv_gc_feature_node_get_actual_access_mode")]
	#[doc(alias = "get_actual_access_mode")]
	fn actual_access_mode(&self) -> GcAccessMode;

	#[doc(alias = "arv_gc_feature_node_get_description")]
	#[doc(alias = "get_description")]
	fn description(&self) -> Option<glib::GString>;

	#[doc(alias = "arv_gc_feature_node_get_display_name")]
	#[doc(alias = "get_display_name")]
	fn display_name(&self) -> Option<glib::GString>;

	/// Gets feature node imposed access mode property.
	///
	/// `<warning>``<para>`Note that this function will not give the actual access mode. Please use
	/// `arv_gc_feature_node_get_actual_access_mode` to get an access mode combined from imposed access
	/// mode and underlying register access mode properties.`</para>``</warning>`
	///
	/// # Returns
	///
	/// Access mode as [`GcAccessMode`][crate::GcAccessMode]
	#[doc(alias = "arv_gc_feature_node_get_imposed_access_mode")]
	#[doc(alias = "get_imposed_access_mode")]
	fn imposed_access_mode(&self) -> GcAccessMode;

	#[doc(alias = "arv_gc_feature_node_get_name")]
	#[doc(alias = "get_name")]
	fn name(&self) -> Option<glib::GString>;

	/// Get feature name space.
	///
	/// # Returns
	///
	/// Name space value as [`GcNameSpace`][crate::GcNameSpace].
	#[doc(alias = "arv_gc_feature_node_get_name_space")]
	#[doc(alias = "get_name_space")]
	fn name_space(&self) -> GcNameSpace;

	#[doc(alias = "arv_gc_feature_node_get_tooltip")]
	#[doc(alias = "get_tooltip")]
	fn tooltip(&self) -> Option<glib::GString>;

	/// Retrieve the node value a string.
	///
	/// `<warning>``<para>`Please note the string content is still owned by the `node` object, which means the returned pointer may not be still valid after a new call to this function.`</para>``</warning>`
	///
	/// # Returns
	///
	/// a string representation of the node value, [`None`] if not applicable.
	#[doc(alias = "arv_gc_feature_node_get_value_as_string")]
	#[doc(alias = "get_value_as_string")]
	fn value_as_string(&self) -> Result<glib::GString, glib::Error>;

	#[doc(alias = "arv_gc_feature_node_get_visibility")]
	#[doc(alias = "get_visibility")]
	fn visibility(&self) -> GcVisibility;

	#[doc(alias = "arv_gc_feature_node_is_available")]
	fn is_available(&self) -> Result<bool, glib::Error>;

	#[doc(alias = "arv_gc_feature_node_is_implemented")]
	fn is_implemented(&self) -> Result<bool, glib::Error>;

	#[doc(alias = "arv_gc_feature_node_is_locked")]
	fn is_locked(&self) -> Result<bool, glib::Error>;

	/// Set the node value using a string representation of the value. May not be applicable to every node type, but safe.
	/// ## `string`
	/// new node value, as string
	#[doc(alias = "arv_gc_feature_node_set_value_from_string")]
	fn set_value_from_string(&self, string: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<GcFeatureNode>> GcFeatureNodeExt for O {
	fn actual_access_mode(&self) -> GcAccessMode {
		unsafe {
			from_glib(ffi::arv_gc_feature_node_get_actual_access_mode(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn description(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_gc_feature_node_get_description(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn display_name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_gc_feature_node_get_display_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn imposed_access_mode(&self) -> GcAccessMode {
		unsafe {
			from_glib(ffi::arv_gc_feature_node_get_imposed_access_mode(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_gc_feature_node_get_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn name_space(&self) -> GcNameSpace {
		unsafe {
			from_glib(ffi::arv_gc_feature_node_get_name_space(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn tooltip(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::arv_gc_feature_node_get_tooltip(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn value_as_string(&self) -> Result<glib::GString, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_gc_feature_node_get_value_as_string(
				self.as_ref().to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib_none(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn visibility(&self) -> GcVisibility {
		unsafe {
			from_glib(ffi::arv_gc_feature_node_get_visibility(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn is_available(&self) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret =
				ffi::arv_gc_feature_node_is_available(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn is_implemented(&self) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret =
				ffi::arv_gc_feature_node_is_implemented(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn is_locked(&self) -> Result<bool, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret =
				ffi::arv_gc_feature_node_is_locked(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(from_glib(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_value_from_string(&self, string: &str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::arv_gc_feature_node_set_value_from_string(
				self.as_ref().to_glib_none().0,
				string.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl fmt::Display for GcFeatureNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("GcFeatureNode")
	}
}
