#![doc = include_str!("../README.md")]
#![deny(unsafe_code)] // at least for now.. 👻

mod model;
pub mod simple;

pub use model::{Observable, Fraction};