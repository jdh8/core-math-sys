#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

/// Sign of the gamma function, set by [`cr_lgamma`] and [`cr_lgammaf`]
#[unsafe(no_mangle)]
pub static mut signgam: core::ffi::c_int = 0;

mod binary32;
pub use binary32::*;

mod binary64;
pub use binary64::*;

#[cfg(feature = "f16")]
mod binary16;
#[cfg(feature = "f16")]
pub use binary16::*;

#[cfg(feature = "f128")]
mod binary128;
#[cfg(feature = "f128")]
pub use binary128::*;
