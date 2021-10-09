#![cfg_attr(feature = "bench", feature(test))]

pub mod context;
pub mod config;
pub(crate) mod data;
mod ffi;
mod fixed;
mod keycodes;
mod phonetic;
pub mod suggestion;
mod utility;
