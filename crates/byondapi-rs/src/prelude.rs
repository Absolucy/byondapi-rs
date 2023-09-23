//! This prelude exposes all of the common types and functions end libraries
//! will end up using.

// We re-export some types from byondapi_sys that end libraries will end up
// needing.

pub use crate::sys;

// Number types
pub use crate::sys::{s1c, s1cMAX, s1cMIN, s2c, s2cMAX, s2cMIN, s4c, s4cMAX, s4cMIN, u1c};
// pub use crate::sys::u1cMAX;
// pub use crate::sys::u1cMIN;
pub use crate::sys::u2c;
// pub use crate::sys::u2cMAX;
// pub use crate::sys::u2cMIN;
pub use crate::sys::u4c;
// pub use crate::sys::u4cMAX;
// pub use crate::sys::u4cMIN;
pub use crate::sys::u8c;
// pub use crate::sys::u8cMAX;
// pub use crate::sys::u8cMIN;
pub use crate::sys::s8c;
// pub use crate::sys::s8cMAX;
// pub use crate::sys::s8cMIN;

// Other types
pub use byondapi_sys::{
	u4cOrPointer, ByondValueData as InternalByondValueData,
	ByondValueType as InternalByondValueType, CByondValue as InternalByondValue,
	CByondValueList as InternalByondValueList,
};

// As well as our own types.
pub use crate::{
	list::ByondValueList,
	value::{ByondTypeCheck, ByondValue},
};
