#![no_std]

#[cfg_attr(feature = "alloc", macro_use)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate std;

#[cfg(feature = "builder")]
mod builder;

mod parser;
mod util;

#[cfg(feature = "builder")]
pub use builder::ArgsLayoutBuilder;

pub use parser::ArgsLayoutRef;
