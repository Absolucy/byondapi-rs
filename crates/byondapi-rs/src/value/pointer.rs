use super::{ByondTypeCheck, ByondValue};
use crate::{static_global::byond, ByondError, ByondResult};

#[repr(transparent)]
pub struct ByondValuePointer(pub ByondValue);

impl ByondValuePointer {
	/// If the value is actually a pointer, this will wrap it in a comfy type.
	/// Otherwise it fails.
	#[inline]
	pub fn new(value: ByondValue) -> ByondResult<Self> {
		value.try_into()
	}

	/// Read from this pointer and get a new [`ByondValue`]
	pub fn read(&self) -> ByondResult<ByondValue> {
		let mut new_value = ByondValue::new();

		unsafe {
			map_byond_error!(byond().Byond_ReadPointer(&self.0 .0, &mut new_value.0))?;
		}

		Ok(new_value)
	}

	/// Write a [`ByondValue`] through this pointer
	pub fn write(&self, value: &ByondValue) -> ByondResult<()> {
		unsafe { map_byond_error!(byond().Byond_WritePointer(&self.0 .0, &value.0)) }
	}
}

impl TryFrom<ByondValue> for ByondValuePointer {
	type Error = ByondError;

	fn try_from(value: ByondValue) -> ByondResult<Self> {
		if value.is_ptr() {
			Ok(Self(value))
		} else {
			Err(ByondError::InvalidConversion {
				expected: "pointer".into(),
				got: value.get_type().name(),
			})
		}
	}
}
