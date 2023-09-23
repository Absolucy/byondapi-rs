use byondapi::byond_fn;

#[byond_fn]
pub fn basic_macro_func(a: u32, b: f32, c: String, d: bool) -> String {
	format!("a={a}, b={b}, c=\"{c}\", d={d}")
}
