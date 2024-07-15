//! # JSON utilities for Rust.
//!
//! A collection of utilities for working with JSON in Rust.
//! Written for my own convenience, but feel free to use it.
//!
//! ## Features
//! - Read and write JSON files.
//! - Print JSON to the console.
//! - Error handling.

pub mod error;
pub mod file;
mod inner;
pub mod json;
pub mod print;
mod test;

/// Re-export `serde`.
pub use serde;
/// Re-export `serde_json`.
pub use serde_json;

pub mod prelude {
    pub use crate::{
        file::{read_json, write_json},
        print::print_json,
    };
    pub use serde_json::Value;
}
