use crate::{
	byond,
	sys::{u4c, CByondValue},
	ByondError, ByondResult, ByondTypeCheck, ByondValue, ByondValueList, ByondValueType, FromByond,
	ToByond,
};
use std::{
	borrow::Cow,
	ffi::{CStr, CString},
	mem::MaybeUninit,
};

/// # Compatibility with the C++ API
impl ByondValue {
	/// This is available for when you really really need access to the raw
	/// [`CByondValue`] but you shouldn't use this normally.
	#[must_use]
	#[inline]
	pub fn into_inner(mut self) -> CByondValue {
		std::mem::replace(&mut self.0, unsafe { std::mem::zeroed() })
	}

	/// Try to get an [`f32`] or fail if this isn't a number type
	#[inline]
	pub fn get_number(&self) -> ByondResult<f32> {
		let val = self.get_type();
		match val {
			ByondValueType::NUMBER => Ok(unsafe { byond().ByondValue_GetNum(&self.0) }),
			_ => Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("number"),
				got: val.name(),
			}),
		}
	}

	/// Try to get a [`String`] or fail if this isn't a string type or isn't
	/// utf8
	#[inline]
	pub fn get_string(&self) -> ByondResult<CString> {
		if !self.is_str() {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("string"),
				got: self.get_type().name(),
			});
		}
		let ptr = unsafe { byond().ByondValue_GetStr(&self.0) };
		let cstr = unsafe { CStr::from_ptr(ptr) };
		Ok(cstr.to_owned())
	}

	/// Get the underlying ref number to this value
	pub fn get_ref(&self) -> ByondResult<u4c> {
		if self.is_str() || self.is_null() || self.is_num() {
			return Err(ByondError::NotReferencable);
		}
		Ok(unsafe { byond().ByondValue_GetRef(&self.0) })
	}
}

/// # In-place modifiers
impl ByondValue {
	/// Replaces whatever is currently in this value with a number
	#[inline]
	pub fn set_number<Num>(&mut self, f: Num)
	where
		Num: Into<f32>,
	{
		unsafe { byond().ByondValue_SetNum(&mut self.0, f.into()) }
	}

	/// Replaces whatever is currently in this value with a string
	pub fn set_str<Str>(&mut self, f: Str) -> ByondResult<()>
	where
		Str: Into<Vec<u8>>,
	{
		let c_string = CString::new(f).map_err(ByondError::boxed)?;
		let c_str = c_string.as_c_str();
		unsafe { map_byond_error!(byond().ByondValue_SetStr(&mut self.0, c_str.as_ptr())) }
	}

	/// Replaces whatever is currently in this value with a ref
	#[inline]
	pub fn set_ref(&mut self, type_: ByondValueType, ref_: u4c) {
		unsafe { byond().ByondValue_SetRef(&mut self.0, *type_, ref_) }
	}
}

/// # Accessors
impl ByondValue {
	#[inline]
	pub fn into_value<Value>(&self) -> ByondResult<Value>
	where
		Value: FromByond,
	{
		Value::from_byond(self)
	}

	/// Read a variable through the ref. Fails if this isn't a ref type.
	pub fn read_var<Name, Value>(&self, name: Name) -> ByondResult<Value>
	where
		Name: Into<Vec<u8>>,
		Value: FromByond,
	{
		let c_string = CString::new(name).map_err(ByondError::boxed)?;
		let c_str = c_string.as_c_str();

		let mut new_value = ByondValue::new();

		unsafe {
			map_byond_error!(byond().Byond_ReadVar(&self.0, c_str.as_ptr(), &mut new_value.0))?;
		}

		Value::from_byond(&new_value).map_err(ByondError::boxed)
	}

	/// Write to a variable through the ref. Fails if this isn't a ref type.
	pub fn write_var<Name, Value>(&mut self, name: Name, value: Value) -> ByondResult<()>
	where
		Name: Into<Vec<u8>>,
		Value: ToByond,
	{
		let c_string = CString::new(name).map_err(ByondError::boxed)?;
		let c_str = c_string.as_c_str();
		let value = value.to_byond()?;

		unsafe { map_byond_error!(byond().Byond_WriteVar(&self.0, c_str.as_ptr(), &value.0)) }
	}

	/// Call a proc using self as src. Fails if this isn't a ref type.
	///
	/// Implicitly set waitfor=0, will never block.
	///
	/// # WARNING FOR BYOND 515.1609 and 515.1610
	/// This is treated as verb name, so underscores are replaced with spaces.
	/// For example `/obj/proc/get_name` would have to be called as
	/// `obj.call("get name")`.
	pub fn call<Name>(&self, name: Name, args: &[ByondValue]) -> ByondResult<ByondValue>
	where
		Name: Into<Vec<u8>>,
	{
		let c_string = CString::new(name).map_err(ByondError::boxed)?;
		let c_str = c_string.as_c_str();

		let str_id = unsafe { byond().Byond_GetStrId(c_str.as_ptr()) };
		if str_id == 0 {
			return Err(ByondError::InvalidProc);
		}

		let ptr = args.as_ptr();
		let mut new_value = ByondValue::new();
		unsafe {
			map_byond_error!(byond().Byond_CallProcByStrId(
				&self.0,
				str_id,
				ptr as *const byondapi_sys::CByondValue,
				args.len() as u32,
				&mut new_value.0
			))?;
		}

		Ok(new_value)
	}

	pub fn path(&self) -> ByondResult<Option<String>> {
		let path = match self.get_type() {
			ByondValueType::OBJ
			| ByondValueType::MOB
			| ByondValueType::AREA
			| ByondValueType::CLIENT
			| ByondValueType::DATUM => self.read_var::<_, ByondValue>("type")?,
			ByondValueType::MOB_TYPEPATH
			| ByondValueType::OBJ_TYPEPATH
			| ByondValueType::TURF_TYPEPATH
			| ByondValueType::AREA_TYPEPATH
			| ByondValueType::DATUM_TYPEPATH => self.clone(),
			ByondValueType::LIST => return Ok(Some("/list".to_string())),
			ByondValueType::IMAGE => return Ok(Some("/image".to_string())),
			ByondValueType::WORLD => return Ok(Some("/world".to_string())),
			_ => return Ok(None),
		};
		unsafe {
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			byond().ByondValue_Init(new_inner.as_mut_ptr());
			map_byond_error!(byond().Byond_ToString(&path.0, new_inner.as_mut_ptr()))?;
			String::from_byond(&Self(new_inner.assume_init())).map(Some)
		}
	}
}

/// # List operations by key instead of indices (why are they even here lumlum?????)
impl ByondValue {
	/// Reads a value by key through the ref. Fails if this isn't a list.
	pub fn read_list_index<Index>(&self, index: Index) -> ByondResult<ByondValue>
	where
		Index: ToByond,
	{
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let index: ByondValue = index.to_byond()?;
		self.read_list_index_internal(&index)
	}

	/// Writes a value by key through the ref. Fails if this isn't a list.
	pub fn write_list_index<Index, Value>(&mut self, index: Index, value: Value) -> ByondResult<()>
	where
		Index: ToByond,
		Value: ToByond,
	{
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		let index: ByondValue = index.to_byond()?;
		let value: ByondValue = value.to_byond()?;
		self.write_list_index_internal(&index, &value)
	}

	/// Reads a value by key through the ref. Fails if the index doesn't exist
	pub fn read_list_index_internal(&self, index: &ByondValue) -> ByondResult<ByondValue> {
		let mut result = ByondValue::new();
		unsafe {
			map_byond_error!(byond().Byond_ReadListIndex(&self.0, &index.0, &mut result.0))?;
		}
		Ok(result)
	}

	/// Writes a value by key through the ref. Dunno why it can fail
	pub fn write_list_index_internal(
		&mut self,
		index: &ByondValue,
		value: &ByondValue,
	) -> ByondResult<()> {
		unsafe {
			map_byond_error!(byond().Byond_WriteListIndex(&self.0, &index.0, &value.0))?;
		}
		Ok(())
	}
}

/// # Builtins
impl ByondValue {
	pub fn builtin_length(&self) -> ByondResult<ByondValue> {
		let mut result = ByondValue::new();
		unsafe {
			map_byond_error!(byond().Byond_Length(&self.0, &mut result.0))?;
		}
		Ok(result)
	}
}

/// # Helpers
impl ByondValue {
	/// Reads a list through the ref. Fails if this isn't a ref type or this
	/// isn't a list.
	#[inline]
	pub fn read_list<Name>(&self, name: Name) -> ByondResult<ByondValueList>
	where
		Name: Into<Vec<u8>>,
	{
		self.read_var::<_, ByondValueList>(name)
	}

	/// Iterates through the assoc values of the list if this value is a list,
	/// if the value isn't a list then the iterator will be empty.
	/// Non assoc lists will have the second field of the tuple be None always,
	/// and the value in the first field (key, value) for proper assoc lists
	pub fn iter(
		&self,
	) -> Result<impl Iterator<Item = (ByondValue, Option<ByondValue>)> + '_, ByondError> {
		if !self.is_list() {
			return Err(ByondError::NotAList);
		}
		Ok(ListIterator {
			value: self,
			length: self.builtin_length()?.into_value()?,
			ctr: 1,
		})
	}
}

struct ListIterator<'a> {
	value: &'a ByondValue,
	length: u32,
	ctr: u32,
}

impl<'a> Iterator for ListIterator<'a> {
	type Item = (ByondValue, Option<ByondValue>);

	fn next(&mut self) -> Option<Self::Item> {
		if self.ctr > self.length {
			return None;
		}
		let ctr = self.ctr.to_byond().ok()?;
		let key = self.value.read_list_index_internal(&ctr).ok()?;
		let value = self
			.value
			.read_list_index_internal(&key)
			.ok()
			.filter(|value| !value.is_null());
		self.ctr += 1;
		Some((key, value))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.length as usize))
	}

	#[inline]
	fn count(self) -> usize {
		self.length as usize
	}
}
