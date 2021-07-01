core+
=====

The batteries for core that you never knew you needed: core+

[![Crates.io](https://img.shields.io/crates/v/coreplus)](https://crates.io/crates/coreplus)
[![docs.rs](https://img.shields.io/docsrs/coreplus.svg)](https://docs.rs/coreplus)
![License](https://img.shields.io/crates/l/coreplus.svg)

* [`coreplus` documentation](https://docs.rs/coreplus)

Core+ contains types that make it possible to write `no_std` libraries that
are fully generic on the network and I/O stack.

## Using the standard library
This crate can support standard library types and traits by enabling the `std` feature.

```toml
[dependencies]
coreplus = { version = "0.2.1", features = ["std"] }
```
