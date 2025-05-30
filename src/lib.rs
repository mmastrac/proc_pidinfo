#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(target_os = "macos"),
    doc = "NOTE: This library is only supported on macOS and iOS."
)]

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use darwin::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;
