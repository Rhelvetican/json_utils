//! # JSON utilities for Rust.
//!
//! A collection of utilities for working with JSON in Rust.
//! Written for my own convenience, but feel free to use it.

pub mod json;
mod print;
pub mod rw;

pub use anyhow::*;

pub mod prelude {
    pub use crate::{
        print::{print_json, print_json_with_indent},
        rw::{read_json, write_json},
    };
    pub use serde_json::Value;
}
