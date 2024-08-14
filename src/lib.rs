#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
