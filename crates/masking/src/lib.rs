// FIXME(kos): Feature `doc_cfg` does require explicit attributes
// `#[doc(cfg(feature = ...))]` for the badges to appear.
// https://doc.rust-lang.org/stable/unstable-book/language-features/doc-cfg.html
// Rather use `doc_auto_cfg` feature, which generates badges from
// the `#[cfg(feature = ...)]` attributes directly:
// https://doc.rust-lang.org/stable/unstable-book/language-features/doc-auto-cfg.html
// https://github.com/rust-lang/rust/pull/90502

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    clippy::expect_used,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used
)]

//!
//! Personal Identifiable Information protection. Wrapper types and traits for secret management which help ensure they aren't accidentally copied, logged, or otherwise exposed (as much as possible), and also ensure secrets are securely wiped from memory when dropped.
//! Secret-keeping library inspired by secrecy.
//!

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR" ), "/", "README.md"))]

pub use zeroize::{self, DefaultIsZeroes, Zeroize as ZeroizableSecret};

mod strategy;

pub use strategy::{Strategy, WithType, WithoutType};
mod abs;
pub use abs::{ExposeInterface, PeekInterface, PeekOptionInterface};

mod secret;
mod strong_secret;
pub use secret::Secret;
pub use strong_secret::StrongSecret;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod boxed;

#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "bytes")]
pub use self::bytes::SecretBytesMut;

#[cfg(feature = "alloc")]
mod string;
#[cfg(feature = "alloc")]
mod vec;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::{Deserialize, SerializableSecret, Serialize};

/// This module should be included with asterisk.
///
/// `use masking::prelude::*;`
///
pub mod prelude {
    pub use super::{ExposeInterface, PeekInterface, PeekOptionInterface};
}

#[cfg(feature = "diesel")]
mod diesel;
