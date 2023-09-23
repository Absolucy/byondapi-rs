use crate::{ByondResult, ByondValue, ByondValueList, ToByond};

impl ToByond for ByondValueList {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.try_into()
	}
}

impl ToByond for &ByondValueList {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(*self).try_into()
	}
}
