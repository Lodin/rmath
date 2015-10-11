#![crate_name="rmath"]
#![crate_type="lib"]
#![crate_type="rlib"]

extern crate iterator2d;
extern crate num;
extern crate rand;

pub mod structs;
pub mod traits;

pub use self::structs::*;
pub use self::traits::*;
