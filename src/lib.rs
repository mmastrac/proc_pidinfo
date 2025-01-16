#![doc = include_str!("../README.md")]
#![cfg_attr(
    not(target_os = "macos"),
    doc = "NOTE: This library is only supported on macOS."
)]

#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "macos")]
mod darwin;
