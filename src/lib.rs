#![cfg_attr(
    not(target_vendor = "apple"),
    doc = "NOTE: This library is only supported on macOS, iOS and other Apple platforms."
)]

#![cfg_attr(target_vendor = "apple", doc = include_str!("../README.md"))]

#[cfg(target_vendor = "apple")]
pub use darwin::*;

#[cfg(target_vendor = "apple")]
mod darwin;
