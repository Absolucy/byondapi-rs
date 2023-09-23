#![allow(clippy::missing_safety_doc)]
pub mod macro_test;

use byondapi::{
	byondval,
	list::ByondValueList,
	map::{byond_block, byond_length, ByondXYZ},
	parse_args,
	value::{pointer::ByondValuePointer, ByondTypeCheck, ByondValue},
	FromByond, ToByond,
};

#[allow(dead_code)]
fn write_log<T: AsRef<[u8]>>(x: T) {
	std::fs::write("./rust_log.txt", x).unwrap()
}

use std::panic;
fn setup_panic_handler() {
	panic::set_hook(Box::new(|info| {
		write_log(format!("Panic {:#?}", info));
	}))
}

#[no_mangle]
pub unsafe extern "C" fn test_connection(
	_argc: byondapi_sys::u4c,
	_argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	ByondValue::new_num(42.0)
}

#[no_mangle]
pub unsafe extern "C" fn test_args(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);
	assert_eq!(args.len(), 1);
	args[0].clone()
}

#[no_mangle]
pub unsafe extern "C" fn send_test(_argc: byondapi_sys::u4c, _argv: *mut ByondValue) -> ByondValue {
	setup_panic_handler();
	// let args = parse_args(argc, argv);
	let new_value = byondval!("Meow");

	std::thread::spawn(move || {
		std::mem::drop(new_value);
	});

	ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_ptr(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);
	let pointer = match ByondValuePointer::new(args[0].clone()) {
		Ok(ptr) => ptr,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	let strobj = match pointer.read() {
		Ok(ptr) => ptr,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	let new_name: ByondValue = byondval!(format!("awa{}", String::from_byond(&strobj).unwrap()));

	match pointer.write(&new_name) {
		Ok(_) => {}
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	ByondValue::null()
}

#[no_mangle]
pub unsafe extern "C" fn test_proc_call(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	// FIXME: Byond will change this in the future
	let result = args[0].call("get name", &[]);

	match result {
		Ok(res) => res,
		Err(e) => byondval!(format!("{:#?}", e)),
	}
}

#[no_mangle]
pub unsafe extern "C" fn test_readwrite_var(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);
	let object = &args[0];

	match object.read_var("name") {
		Ok(res) => res,
		Err(e) => byondval!(format!("{:#?}", e)),
	}
}

#[no_mangle]
pub unsafe extern "C" fn test_list_push(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let mut list: ByondValueList = match (&args[0]).try_into() {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	match list.push(&ByondValue::new_num(8.0)) {
		Ok(_) => {}
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	list.to_byond().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_double(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let list: ByondValueList = match (&args[0]).try_into() {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	let collection: Vec<ByondValue> = list
		.iter()
		.map(|f| byondval!(f.get_number().unwrap() * 2.))
		.collect();

	let list: ByondValueList = collection.as_slice().to_byond().unwrap();

	list.to_byond().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_index(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let list: ByondValueList = match (&args[0]).try_into() {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	list[3].clone()
}

#[no_mangle]
pub unsafe extern "C" fn test_list_pop(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let mut list: ByondValueList = match (&args[0]).try_into() {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	let element = match list.pop() {
		Ok(x) => x,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	if list.0.count != 4 {
		return byondval!("pop did not actually remove item from list");
	}

	element
}

#[no_mangle]
pub unsafe extern "C" fn test_length_with_list(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let list: ByondValueList = match (&args[0]).try_into() {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	let value: ByondValue = match list.try_into() {
		Ok(x) => x,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	match byond_length(&value) {
		Ok(x) => x,
		Err(e) => byondval!(format!("{:#?}", e)),
	}
}

#[no_mangle]
pub unsafe extern "C" fn test_block(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
	setup_panic_handler();
	let _args = parse_args(argc, argv);

	let block = match byond_block(
		ByondXYZ::with_coords((1, 1, 1)),
		ByondXYZ::with_coords((2, 2, 1)),
	) {
		Ok(list) => list,
		Err(e) => return byondval!(format!("{:#?}", e)),
	};

	if block.len() != 4 {
		return byondval!(format!(
			"block returned {} turfs when we expected 4",
			block.len()
		));
	}

	byondval!(block.len() as f32)
}

#[no_mangle]
pub unsafe extern "C" fn test_length_with_str(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	match byond_length(&args[0]) {
		Ok(x) => x,
		Err(e) => byondval!(format!("{:#?}", e)),
	}
}
#[no_mangle]
pub unsafe extern "C" fn test_list_key_lookup(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let mut args = parse_args(argc, argv);

	let list = args.get_mut(0).unwrap();

	let num: f32 = match list.read_list_index("cat") {
		Ok(x) => x.into_value().unwrap(),
		Err(e) => return byondval!(format!("{:#?}", e)),
	};
	assert_eq!(num, 7.0);

	let num: f32 = match list.read_list_index("dog") {
		Ok(x) => x.into_value().unwrap(),
		Err(e) => return byondval!(format!("{:#?}", e)),
	};
	assert_eq!(num, 5.0);

	let num: f32 = match list.read_list_index("parrot") {
		Ok(x) => x.into_value().unwrap(),
		Err(e) => return byondval!(format!("{:#?}", e)),
	};
	assert_eq!(num, 4.0);

	if let Err(e) = list.write_list_index("parrot", 14.0) {
		return byondval!(format!("{:#?}", e));
	};

	let key: String = list.read_list_index(3.0).unwrap().into_value().unwrap();

	assert_eq!("parrot", key);

	let map = list
		.iter()
		.unwrap()
		.map(|(k, v)| {
			(
				k.get_string().unwrap(),
				v.unwrap().get_number().unwrap() as u32,
			)
		})
		.collect::<Vec<_>>();

	assert_eq!(map, vec![
		("cat".to_owned(), 7),
		("dog".to_owned(), 5),
		("parrot".to_owned(), 14)
	]);

	ByondValue::new()
}

#[no_mangle]
pub unsafe extern "C" fn test_ref(argc: byondapi_sys::u4c, argv: *mut ByondValue) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);

	let turf = args.get(0).unwrap();
	let turf_type = turf.get_type();
	let turf_id = turf.get_ref().unwrap();

	byondval!(format!("turf_id: {turf_id}, turf_type: {turf_type:?}"))
}

#[no_mangle]
pub unsafe extern "C" fn test_non_assoc_list(
	argc: byondapi_sys::u4c,
	argv: *mut ByondValue,
) -> ByondValue {
	setup_panic_handler();
	let args = parse_args(argc, argv);
	let list = args.get(0).unwrap();

	let map = list
		.iter()
		.unwrap()
		.map(|(k, v)| {
			if v.map(|value| !value.is_null()).unwrap_or(false) {
				panic!("value is not null")
			}
			k.get_string().unwrap()
		})
		.collect::<Vec<_>>();

	assert_eq!(map, vec![
		"cat".to_owned(),
		"dog".to_owned(),
		"parrot".to_owned()
	]);

	ByondValue::new()
}
