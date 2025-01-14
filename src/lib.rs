#[cfg(target_os = "macos")]
pub use darwin::*;

#[cfg(target_os = "macos")]
mod darwin;
