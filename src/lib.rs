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
//! coreplus = { version = "0.1.0", features = ["std"] }
//! ```
//!
//! ## The unstable feature
//! Enabling the `unstable` feature will add `*_vectored` methods to the [`io::AsyncRead`], [`io::AsyncWrite`],
//! [`io::Read`], and [`io::Write`] traits.
//!
//! ```toml
//! [dependencies]
//! coreplus = { version = "0.1.0", features = ["unstable"] }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", feature(generic_associated_types))]

pub mod io;
pub mod net;
