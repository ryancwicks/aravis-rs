// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Buffer;
use crate::Gc;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::ptr;

glib::wrapper! {
	#[doc(alias = "ArvChunkParser")]
	pub struct ChunkParser(Object<ffi::ArvChunkParser, ffi::ArvChunkParserClass>);

	match fn {
		type_ => || ffi::arv_chunk_parser_get_type(),
	}
}

impl ChunkParser {
	/// Creates a new chunk_parser.
	/// ## `xml`
	/// XML genicam data
	/// ## `size`
	/// genicam data size, -1 if NULL terminated
	///
	/// # Returns
	///
	/// a new [`ChunkParser`][crate::ChunkParser] object
	#[doc(alias = "arv_chunk_parser_new")]
	pub fn new(xml: &str, size: usize) -> ChunkParser {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::arv_chunk_parser_new(xml.to_glib_none().0, size)) }
	}

	/// ## `buffer`
	/// a [`Buffer`][crate::Buffer] with a [`BufferPayloadType::ChunkData`][crate::BufferPayloadType::ChunkData] payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the boolean chunk data value.
	#[doc(alias = "arv_chunk_parser_get_boolean_value")]
	#[doc(alias = "get_boolean_value")]
	pub fn boolean_value(&self, buffer: &Buffer, chunk: &str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::arv_chunk_parser_get_boolean_value(
				self.to_glib_none().0,
				buffer.to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// ## `buffer`
	/// a [`Buffer`][crate::Buffer] with a [`BufferPayloadType::ChunkData`][crate::BufferPayloadType::ChunkData] payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the float chunk data value.
	#[doc(alias = "arv_chunk_parser_get_float_value")]
	#[doc(alias = "get_float_value")]
	pub fn float_value(&self, buffer: &Buffer, chunk: &str) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_chunk_parser_get_float_value(
				self.to_glib_none().0,
				buffer.to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// ## `buffer`
	/// a [`Buffer`][crate::Buffer] with a [`BufferPayloadType::ChunkData`][crate::BufferPayloadType::ChunkData] payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the integer chunk data integer.
	#[doc(alias = "arv_chunk_parser_get_integer_value")]
	#[doc(alias = "get_integer_value")]
	pub fn integer_value(&self, buffer: &Buffer, chunk: &str) -> Result<i64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_chunk_parser_get_integer_value(
				self.to_glib_none().0,
				buffer.to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// ## `buffer`
	/// a [`Buffer`][crate::Buffer] with a [`BufferPayloadType::ChunkData`][crate::BufferPayloadType::ChunkData] payload
	/// ## `chunk`
	/// chunk data name
	///
	/// # Returns
	///
	/// the string chunk data value.
	#[doc(alias = "arv_chunk_parser_get_string_value")]
	#[doc(alias = "get_string_value")]
	pub fn string_value(&self, buffer: &Buffer, chunk: &str) -> Result<glib::GString, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_chunk_parser_get_string_value(
				self.to_glib_none().0,
				buffer.to_glib_none().0,
				chunk.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib_none(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// Internal Genicam object
	pub fn genicam(&self) -> Option<Gc> {
		unsafe {
			let mut value = glib::Value::from_type(<Gc as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.as_ptr() as *mut glib::gobject_ffi::GObject,
				b"genicam\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value
				.get()
				.expect("Return Value for property `genicam` getter")
		}
	}
}

unsafe impl Send for ChunkParser {}

impl fmt::Display for ChunkParser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("ChunkParser")
	}
}