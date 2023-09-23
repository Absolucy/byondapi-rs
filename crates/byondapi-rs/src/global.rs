use crate::{static_global::byond, sys::CByondValue, ByondError, ByondResult, ByondValue};
use std::ffi::CString;

/// Call a global proc.
///
/// Implicitly set waitfor=0, will never block.
///
/// # WARNING
/// This is treated as verb name, so underscores are replaced with spaces.
/// For example `/obj/proc/get_name` would have to be called as
/// `obj.call("get name")`.
pub fn call_global_proc<Name>(name: Name, args: &[ByondValue]) -> ByondResult<ByondValue>
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
		map_byond_error!(byond().Byond_CallGlobalProcByStrId(
			str_id,
			ptr as *const CByondValue,
			args.len() as u32,
			&mut new_value.0
		))?;
	}

	Ok(new_value)
}
