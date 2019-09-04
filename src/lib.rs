//! Provides macros for fetching the Cargo package version at compile time.
//!
//! All macros defined by this crate return constant expressions, so they can be used inside
//! `const fn`s or to initialize the value of a `const` or `static` item.
//!
//! # Example
//!
//! ```
//! use pkg_version::*;
//!
//! const MAJOR: u32 = pkg_version_major!();
//! const MINOR: u32 = pkg_version_minor!();
//! const PATCH: u32 = pkg_version_patch!();
//!
//! fn main() {
//!     let version = format!("{}.{}.{}", MAJOR, MINOR, PATCH);
//!     assert_eq!(version, "0.1.0");
//!
//!     println!("I am version {}", version);
//! }
//! ```

#![no_std]

#![doc(html_root_url = "https://docs.rs/pkg-version/0.1.0")]
#![warn(missing_debug_implementations, rust_2018_idioms)]

use proc_macro_hack::proc_macro_hack;

/// Expands to the major version number of the Cargo package, as an integer literal.
#[proc_macro_hack]
pub use pkg_version_impl::pkg_version_major;

/// Expands to the minor version number of the Cargo package, as an integer literal.
#[proc_macro_hack]
pub use pkg_version_impl::pkg_version_minor;

/// Expands to the patch version number of the Cargo package, as an integer literal.
#[proc_macro_hack]
pub use pkg_version_impl::pkg_version_patch;
