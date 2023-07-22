#[macro_use]
extern crate lazy_static;

mod static_global;

pub mod error;
pub use error::Error;

pub mod list;
pub mod typecheck_trait;
pub mod value;