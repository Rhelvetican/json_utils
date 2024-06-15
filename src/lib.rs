//! # JSON utilities for Rust.
//!
//! A collection of utilities for working with JSON in Rust.
//! Written for my own convenience, but feel free to use it.

pub mod file;
pub mod json;
pub mod print;

pub use anyhow::*;

const INDENT: &[u8] = b"    ";

pub mod prelude {
    pub use crate::{
        file::{read_json, write_json},
        print::{print_json, print_json_with_indent},
    };
    pub use serde_json::Value;
}
