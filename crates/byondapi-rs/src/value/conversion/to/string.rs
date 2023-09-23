use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::{
	borrow::Cow,
	ffi::{CStr, CString, OsStr, OsString},
	path::{Path, PathBuf},
};

impl ToByond for &String {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		ByondValue::new_str(self.as_str())
	}
}

impl ToByond for String {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(&self).to_byond()
	}
}

impl ToByond for &str {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		ByondValue::new_str(*self)
	}
}

impl<'a> ToByond for Cow<'a, str> {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		ByondValue::new_str(self.as_ref())
	}
}

impl ToByond for &Path {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let value = self
			.to_str()
			.map(ByondValue::new_str)
			.ok_or(ByondError::NonUtf8String)??;
		Ok(value)
	}
}

impl ToByond for PathBuf {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_path().to_byond()
	}
}

impl ToByond for CString {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		ByondValue::new_str(self.as_bytes())
	}
}

impl ToByond for &CStr {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		ByondValue::new_str(self.to_bytes())
	}
}

impl ToByond for OsString {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_os_str().to_byond()
	}
}

impl ToByond for &OsStr {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		let value = self
			.to_str()
			.map(ByondValue::new_str)
			.ok_or(ByondError::NonUtf8String)??;
		Ok(value)
	}
}
