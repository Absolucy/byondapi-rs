use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, ReturnType};

#[proc_macro_attribute]
pub fn byond_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let func = parse_macro_input!(item as ItemFn);

	let func_name = &func.sig.ident;
	let wrapper_name = format!("__byond_{}_inner", func_name);
	let wrapper_ident = syn::Ident::new(&wrapper_name, func_name.span());

	let mod_name = format!("__byond_export_{}", func_name);
	let mod_ident = syn::Ident::new(&mod_name, func_name.span());

	let mut parse_args = Vec::new();

	let length = func.sig.inputs.len();
	for (idx, input) in func.sig.inputs.iter().enumerate() {
		if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
			let mutability = attrs.iter().find(|attr| attr.path().is_ident("mut"));
			parse_args.push(quote! {
				let #mutability #pat: #ty = ::byondapi::FromByond::from_byond(&__byond_args[#idx]).expect("failed to parse argument");
			});
		}
	}

	let return_type: TokenStream2;
	let return_conversion = match &func.sig.output {
		ReturnType::Default => {
			return_type = quote!(());
			quote! {
				Ok(::byondapi::ByondValue::null())
			}
		}
		ReturnType::Type(_, ty) => {
			return_type = quote!(#ty);
			let ty_name = quote!(#ty).to_string();
			if ty_name.contains("Result") {
				quote! {
					ret
						.map_err(::std::boxed::Box::from)
						.and_then(|inner_ret| ::byondapi::ByondValue::new_value(inner_ret).map_err(::std::boxed::Box::from))
				}
			} else {
				quote! {
					::byondapi::ByondValue::new_value(ret).map_err(::std::boxed::Box::from)
				}
			}
		}
	};
	let body = &func.block;

	let gen = quote! {
		#func

		#[doc(hidden)]
		mod #mod_ident {
			use super::*;

			#[no_mangle]
			#[inline(never)]
			pub unsafe extern "C" fn #func_name(__argc: ::byondapi::sys::u4c, __argv: *mut ::byondapi::ByondValue) -> ::byondapi::ByondValue {
				::byondapi::panic::setup_panic_hook();
				let mut __args = unsafe { ::byondapi::parse_args(__argc, __argv) };
				if __args.len() < #length {
					__args.extend((0..#length - __args.len()).map(|_| ::byondapi::ByondValue::default()))
				}

				fn #wrapper_ident(mut __byond_args: &[::byondapi::ByondValue]) -> ::std::result::Result<::byondapi::ByondValue, ::std::boxed::Box<dyn ::std::error::Error>> {
					#(#parse_args)*

					let ret: #return_type = {
						#body
					};

					#return_conversion
				}

				let __ret = match ::std::panic::catch_unwind(|| #wrapper_ident(&__args)) {
					Ok(value) => value,
					Err(_err) => return ::byondapi::panic::get_last_panic(),
				};

				match __ret {
					Ok(value) => value,
					Err(err)=> ::byondapi::create_error(err),
				}
			}
		}
	};
	gen.into()
}
