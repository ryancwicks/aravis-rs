// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Stream;
use std::fmt;

glib::wrapper! {
	#[doc(alias = "ArvUvStream")]
	pub struct UvStream(Object<ffi::ArvUvStream, ffi::ArvUvStreamClass>) @extends Stream;

	match fn {
		type_ => || ffi::arv_uv_stream_get_type(),
	}
}

impl UvStream {}

unsafe impl Send for UvStream {}

impl fmt::Display for UvStream {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("UvStream")
	}
}
