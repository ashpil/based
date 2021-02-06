#![cfg_attr(feature = "generic", feature(min_const_generics))]

#[cfg(not(feature = "generic"))]
pub mod vec;

#[cfg(feature = "generic")]
pub mod vecgen;
#[cfg(feature = "generic")]
pub use vecgen as vec;

