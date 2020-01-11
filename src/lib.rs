// lib.rs
//
//! A single-pass diff-based compiler for the C, Rust, Python and Aratar
//! programming languages.

#[cfg(feature = "rust")]
pub mod rust;
#[cfg(feature = "c")]
pub mod c;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "aratar")]
pub mod aratar;

mod lexeme;

pub use lexeme::*;
