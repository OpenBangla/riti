#![cfg_attr(feature = "bench", feature(test))]

pub mod config;
pub mod context;
pub(crate) mod data;
mod ffi;
mod fixed;
pub mod keycodes;
mod khipro;
mod phonetic;
pub mod suggestion;
mod utility;
