#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::perf,
	clippy::style,
	clippy::suspicious
)]
#![allow(clippy::tabs_in_doc_comments)]
mod static_global;

#[macro_use]
pub mod error;
#[cfg(feature = "byond-515-1617")]
pub mod global;
pub mod list;
#[cfg(feature = "byond-515-1611")]
pub mod map;
#[doc(hidden)]
pub mod panic;
pub mod prelude;
pub mod value;

#[cfg(feature = "byond-515-1617")]
pub use crate::global::call_global_proc;
#[cfg(feature = "byond-515-1611")]
pub use crate::map::ByondXYZ;
pub use crate::{
	error::{ByondError, ByondResult},
	list::ByondValueList,
	static_global::byond,
	value::{
		conversion::{FromByond, ToByond},
		typecheck::{ByondTypeCheck, ByondValueType},
		ByondValue,
	},
};
pub use byondapi_impl::byond_fn;
use std::cell::RefCell;

thread_local! {
	static ERROR_TYPEPATH: RefCell<ByondValue> = RefCell::new(byondval!("/datum/ext_error"));
}

/// A simple macro to create a [`ByondValue`] from any Rust value that
/// implements Into<ByondValue>.
#[macro_export]
macro_rules! byondval {
	(const $value:expr) => {{
		thread_local! {
			static __BYONDVAL: ::std::cell::RefCell<$crate::value::ByondValue> = ::std::cell::RefCell::new($crate::byondval!($value));
		}
		__BYONDVAL.with(|v| v.borrow().clone())
	}};
	($value:expr) => {
		$crate::ToByond::to_byond(&$value).unwrap()
	};
}

/// # Safety
/// Don't pass in a null argv pointer please god
/// Just give this what BYOND gives you and pray for the best
pub unsafe fn parse_args(argc: sys::u4c, argv: *mut ByondValue) -> Vec<ByondValue> {
	if argc == 0 || argv.is_null() {
		return Vec::new();
	}
	unsafe { std::slice::from_raw_parts_mut(argv, argc as usize).to_vec() }
}

#[cfg(feature = "byond-515-1617")]
pub fn create_error(error: Box<dyn std::error::Error>) -> ByondValue {
	ERROR_TYPEPATH.with(|path| {
		ByondValue::new_type(&path.borrow(), &[byondval!(format!("{:?}", error))])
			.unwrap_or_default()
	})
}

#[cfg(not(feature = "byond-515-1617"))]
#[inline]
pub fn create_error(error: Box<dyn std::error::Error>) -> ByondValue {
	ByondValue::null()
}

/// Re-export of byondapi_sys for all low level things you may run into.
pub mod sys {
	pub use byondapi_sys::*;
}
