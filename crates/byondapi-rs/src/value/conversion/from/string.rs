use crate::{ByondResult, ByondValue, FromByond};
use std::{ffi::CString, path::PathBuf};

impl FromByond for CString {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value.get_string()
	}
}

impl FromByond for String {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.get_string()
			.map(|string| string.to_string_lossy().into_owned())
	}
}

impl FromByond for PathBuf {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		String::from_byond(value).map(PathBuf::from)
	}
}
