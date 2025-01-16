#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
///! NOTE: This library is only supported on macOS.

#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "macos")]
mod darwin;
