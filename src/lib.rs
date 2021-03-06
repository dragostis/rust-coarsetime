
//! A crate to make time measurements that focuses on speed.
//!
//! This crate is a partial replacement for the `Time` and `Duration` structures from the
//! standard library, with the following differences:
//!
//! * Speed is privileged over accuracy. In particular, `CLOCK_MONOTONIC_COARSE` is used to
//! retrieve the clock value on Linux systems, and transformations avoid operations that can be
//! slow on non-Intel systems.
//! * The number of system calls can be kept to a minimum. The "most recent timestamp" is
//! always kept in memory. It can be read with just a load operation, and can be updated only as
//! frequently as necessary.
//!
//! # Installation
//!
//! `coarsetime` is available on [crates.io](https://crates.io/crates/coarsetime) and works on
//! Rust stable, beta, and nightly.
//!
//! Windows and Unix-like systems are supported.
//!
//! Available features:
//!
//! * `nightly`: optimizes for rust-nightly

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "nightly", feature(const_fn))]
#![cfg_attr(feature = "nightly", feature(integer_atomics))]
#![cfg_attr(feature = "nightly", feature(const_atomic_u64_new))]
#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[macro_use]
extern crate lazy_static;
extern crate libc;

pub use clock::*;
pub use duration::*;
pub use instant::*;
pub use updater::*;

mod clock;
mod duration;
mod helpers;
mod instant;
mod updater;

#[cfg(test)]
mod tests;
