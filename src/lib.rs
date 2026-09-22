#![doc = include_str!("../README.md")]
#![deny(unsafe_code)] // at least for now.. 👻

pub mod feature;
mod model;
mod observation;
#[allow(dead_code)] // TODO: figure out if the similarity lives in this crate.
pub(crate) mod similarity;
#[allow(dead_code)] // TODO: remove after finalizing the temporal bits.
pub(crate) mod temporal;

pub use model::Fraction;
pub use observation::{Observable, ObservableFeatures};
