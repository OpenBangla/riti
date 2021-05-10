#![cfg_attr(feature = "bench", feature(test))]
#![allow(dead_code)]

pub mod context;
pub mod config;
mod ffi;
mod fixed;
mod keycodes;
mod loader;
mod phonetic;
pub mod suggestion;
mod utility;
