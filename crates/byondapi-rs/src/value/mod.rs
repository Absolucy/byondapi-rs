//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]

use byondapi_sys::CByondValue;

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern over [`CByondValue`]
#[repr(transparent)]
pub struct ByondValue(pub CByondValue);

/// It is safe to send ByondValue with ownership, but it is not safe to have
/// references between threads.
unsafe impl Send for ByondValue {}

pub mod constructors;
pub mod conversion;
pub mod functions;
pub mod pointer;
pub mod trait_impls;
pub mod typecheck;

pub use conversion::*;
pub use typecheck::{ByondTypeCheck, ByondValueType};

#[doc(hidden)]
pub use backtrace;
