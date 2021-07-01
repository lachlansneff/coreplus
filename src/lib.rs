//! The batteries for core that you never knew you needed: core+.
//!
//! This crate contains a number of traits that make it possible to write
//! libraries that are fully generic on the network and I/O stack.
//!
//! ## Using the standard library
//! This crate can be used with standard library types by enabling the `std` feature.
//!
//! ```toml
//! [dependencies]
//! coreplus = { version = "0.2.0", features = ["std"] }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod io;
pub mod net;
