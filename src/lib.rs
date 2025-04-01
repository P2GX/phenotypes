#![doc = include_str!("../README.md")]
#![deny(unsafe_code)] // at least for now.. 👻

mod model;
mod observation;
pub mod simple;

pub use model::Fraction;
pub use observation::{Observable, ObservableFeatures};
