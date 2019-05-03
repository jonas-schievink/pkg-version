# Access the Cargo package version in constants

[![crates.io](https://img.shields.io/crates/v/pkg-version.svg)](https://crates.io/crates/pkg-version)
[![docs.rs](https://docs.rs/pkg-version/badge.svg)](https://docs.rs/pkg-version/)
[![Build Status](https://travis-ci.org/jonas-schievink/pkg-version.svg?branch=master)](https://travis-ci.org/jonas-schievink/pkg-version)

This crate provides macros (`pkg_verison_major!`, etc.) that expand to the Cargo
package version, parsed into an integer literal.

Previously, the only way to access the package version was by using
`env!("CARGO_PKG_VERSION_MAJOR")` etc., but doing that always results in a
*string* literal, which can only be parsed into a number at runtime. This crate
fixes that problem by parsing the version during macro expansion.

Please refer to the [changelog](CHANGELOG.md) to see what changed in the last
releases.

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
pkg-version = "0.0.0"
```

Check the [API Documentation](https://docs.rs/pkg-version/) for how to use the
crate's functionality.
