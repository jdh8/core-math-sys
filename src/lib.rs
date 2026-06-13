#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "f16")]
mod binary16;
#[cfg(feature = "f16")]
pub use binary16::*;

#[cfg(feature = "f128")]
mod binary128;
#[cfg(feature = "f128")]
pub use binary128::*;
