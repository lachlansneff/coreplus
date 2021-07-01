core+
=====

The batteries for core that you never knew you needed: core+

[![Crates.io](https://img.shields.io/crates/v/coreplus)](https://crates.io/crates/coreplus)
[![docs.rs](https://img.shields.io/docsrs/coreplus)](https://docs.rs/coreplus)
![License](https://img.shields.io/crates/l/coreplus)

Core+ contains traits that make it possible to write `no_std` libraries that
are fully generic on the network and I/O stack.

## `no_std`
This crate can be used without the standard library by disabling the `std` feature.

```toml
[dependencies]
coreplus = { version = "0.1.0", default-features = false }
```

## The unstable feature
Enabling the `unstable` feature will add `*_vectored` methods to the [`io::AsyncRead`], [`io::AsyncWrite`],
[`io::Read`], and [`io::Write`] traits.

