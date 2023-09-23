use crate::sys::ByondApi;
use std::sync::OnceLock;

#[cfg(target_os = "windows")]
pub fn init_lib() -> ByondApi {
	let library = {
		let result = libloading::os::windows::Library::open_already_loaded("byondcore.dll");

		match result {
			Ok(lib) => lib,
			Err(err) => {
				let message =
					format!("byondcore.dll is not loaded into the process as expected: {err:#?}",);
				crate::error::crash_logging::log_to_file(&message);
				panic!("{message}")
			}
		}
	};

	unsafe { ByondApi::init_from_library(library) }.expect("Failed to initialize library.")
}

#[cfg(target_os = "linux")]
pub fn init_lib() -> ByondApi {
	let library = libloading::os::unix::Library::this();
	match unsafe { ByondApi::init_from_library(library) } {
		Err(err) => {
			let message =
				format!("libbyond.so is not loaded into the process as expected: {err:#?}",);
			crate::error::crash_logging::log_to_file(&message);
			panic!("{message}")
		}
		Ok(res) => res,
	}
}

static BYOND: OnceLock<ByondApi> = OnceLock::new();

#[inline(always)]
pub fn byond() -> &'static ByondApi {
	BYOND.get_or_init(init_lib)
}
