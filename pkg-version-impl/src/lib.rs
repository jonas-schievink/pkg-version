//! Implementation details of the [`pkg-version`] crate.
//!
//! Do not use this crate directly. It does not provide a stable API and might
//! break at any time. Use [`pkg-version`] instead.
//!
//! [`pkg-version`]: https://docs.rs/pkg-version

#![doc(html_root_url = "https://docs.rs/pkg-version-impl/0.0.0")]
#![warn(missing_debug_implementations, rust_2018_idioms)]

#[allow(unused_extern_crates)] // ignore warning on nightly
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::env;

/// A type large enough to hold a version component.
///
/// This should match what the `semver` crate uses.
type VersionNum = u64;

#[proc_macro_hack]
pub fn pkg_version_major(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("unexpected arguments for `pkg_version_major!` macro (expected no arguments)");
    }

    let version = env::var("CARGO_PKG_VERSION_MAJOR")
        .unwrap()
        .parse::<VersionNum>()
        .unwrap();

    version.to_string().parse().unwrap()
}

#[proc_macro_hack]
pub fn pkg_version_minor(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("unexpected arguments for `pkg_version_minor!` macro (expected no arguments)");
    }

    let version = env::var("CARGO_PKG_VERSION_MINOR")
        .unwrap()
        .parse::<VersionNum>()
        .unwrap();

    version.to_string().parse().unwrap()
}

#[proc_macro_hack]
pub fn pkg_version_patch(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("unexpected arguments for `pkg_version_patch!` macro (expected no arguments)");
    }

    let version = env::var("CARGO_PKG_VERSION_PATCH")
        .unwrap()
        .parse::<VersionNum>()
        .unwrap();

    version.to_string().parse().unwrap()
}
