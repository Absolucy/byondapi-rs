use crate::{
	prelude::ByondTypeCheck,
	static_global::byond,
	sys::CByondValue,
	value::{ByondValue, ByondValueType},
};
use std::{
	fmt::Debug,
	hash::{Hash, Hasher},
	mem::MaybeUninit,
};

// Memory handling
impl Clone for ByondValue {
	fn clone(&self) -> Self {
		unsafe {
			let mut new_inner = MaybeUninit::<CByondValue>::uninit();
			// Safety: new_inner is going to an initialization function, it will only write
			// to the pointer.
			byond().ByondValue_CopyFrom(new_inner.as_mut_ptr(), &self.0);
			// Safety: ByondValue_Init will have initialized the new_inner.
			Self(new_inner.assume_init())
		}
	}
}

impl Drop for ByondValue {
	#[inline]
	fn drop(&mut self) {
		// Safety: We are being dropped, it is okay to free our inner CByondValue.
		unsafe { byond().ByondValue_Free(&mut self.0) }
	}
}

// Equality
impl PartialEq for ByondValue {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		// Safety: This operation only fails if our CByondValue is invalid, which cannot
		// happen.
		unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
	}
}

impl Eq for ByondValue {}

impl Hash for ByondValue {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		let ty = self.get_type();
		state.write_u8(*ty);
		unsafe {
			match ty {
				ByondValueType::STRING => {
					let str_ptr = byond().ByondValue_GetStr(&self.0);
					let cstr = std::ffi::CStr::from_ptr(str_ptr);
					state.write(cstr.to_bytes());
				}
				ByondValueType::NUMBER => {
					let num = byond().ByondValue_GetNum(&self.0);
					state.write_u32(num.to_bits());
				}
				ByondValueType::NULL => {}
				_ => {
					state.write_u32(self.0.data.ref_);
				}
			}
		}
	}
}

// Debug!
impl Debug for ByondValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let typ = format!("{:X}", self.0.type_);
		let value = format!("{:X}", unsafe { self.0.data.ref_ });

		f.debug_tuple("ByondValue")
			.field(&typ)
			.field(&value)
			.finish()
	}
}
