use crate::{
	static_global::byond,
	sys::{u4c, CByondValue},
	ByondError, ByondResult, ByondTypeCheck, ByondValue, ByondValueType, ToByond,
};
use std::{borrow::Cow, ffi::CString, mem::MaybeUninit};

impl Default for ByondValue {
	fn default() -> Self {
		unsafe {
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			// Safety: new_inner is going to an initialization function, it will only write
			// to the pointer.
			byond().ByondValue_Init(new_inner.as_mut_ptr());
			// Safety: ByondValue_Init will have initialized the new_inner.
			Self(new_inner.assume_init())
		}
	}
}

/// # Constructors
impl ByondValue {
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}

	#[inline]
	pub fn null() -> Self {
		Self::default()
	}

	#[inline]
	pub fn new_value<Value>(value: Value) -> ByondResult<Self>
	where
		Value: ToByond,
	{
		value.to_byond()
	}

	#[cfg(feature = "byond-515-1617")] // Technically available in earlier API versions, but broken until 1617.
	pub fn new_type(typepath: &ByondValue, args: &[ByondValue]) -> ByondResult<Self> {
		let value_type = typepath.get_type();
		if !matches!(
			value_type,
			ByondValueType::STRING
				| ByondValueType::MOB_TYPEPATH
				| ByondValueType::OBJ_TYPEPATH
				| ByondValueType::TURF_TYPEPATH
				| ByondValueType::AREA_TYPEPATH
				| ByondValueType::DATUM_TYPEPATH,
		) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("typepath or string"),
				got: value_type.name(),
			});
		}
		unsafe {
			let mut value = MaybeUninit::<CByondValue>::uninit();
			// Safety: value is going to an initialization function, it will only write
			// to the pointer.
			byond().ByondValue_Init(value.as_mut_ptr());
			// Safety: ByondValue_Init will have initialized the value.
			map_byond_error!(byond().Byond_New(
				&typepath.0,
				args.as_ptr() as _,
				args.len() as _,
				value.as_mut_ptr(),
			))?;
			// Safety: ByondValue_Init will have initialized the new_inner.
			Ok(Self(value.assume_init()))
		}
	}

	pub fn new_ref(typ: ByondValueType, ptr: u4c) -> Self {
		unsafe {
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			// Safety: new_inner is going to an initialization function, it will only write
			// to the pointer.
			byond().ByondValue_InitRef(new_inner.as_mut_ptr(), *typ, ptr);
			// Safety: ByondValue_Init will have initialized the new_inner.
			Self(new_inner.assume_init())
		}
	}

	pub fn new_num(f: f32) -> Self {
		unsafe {
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			// Safety: new_inner is going to an initialization function, it will only write
			// to the pointer.
			byond().ByondValue_InitNum(new_inner.as_mut_ptr(), f);
			// Safety: ByondValue_Init will have initialized the new_inner.
			Self(new_inner.assume_init())
		}
	}

	pub fn new_str<Str>(s: Str) -> ByondResult<Self>
	where
		Str: Into<Vec<u8>>,
	{
		unsafe {
			let c_str = CString::new(s.into()).map_err(|_| ByondError::NonUtf8String)?;
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			map_byond_error!(byond().ByondValue_InitStr(new_inner.as_mut_ptr(), c_str.as_ptr()))?;
			Ok(Self(new_inner.assume_init()))
		}
	}

	pub fn new_list() -> ByondResult<Self> {
		let mut new_self = Self::new();
		unsafe { map_byond_error!(byond().Byond_CreateList(&mut new_self.0))? }
		Ok(new_self)
	}
}

impl<'a> ByondValue {
	/// # Safety
	/// The [`CByondValue`] must be initialized.
	#[inline]
	pub unsafe fn from_ref(s: &'a CByondValue) -> &'a Self {
		unsafe { std::mem::transmute(s) }
	}
}
