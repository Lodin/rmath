#![crate_name="rmath"]
#![crate_type="lib"]
#![crate_type="rlib"]

extern crate iterator2d;
extern crate num;

pub mod macros;
pub mod structs;
pub mod traits;

pub use self::macros::*;
pub use self::structs::*;
pub use self::traits::*;
