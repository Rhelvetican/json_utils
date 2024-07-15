//! # Read & Write JSON Utilities
//!
//! A collection of utilities for reading and writing JSON files.
//!
use std::{fs::write, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, ser::PrettyFormatter, Serializer};

use crate::{
    error::Error,
    inner::{_create_parent_dir, _read_json_inner},
};

/// Reads a JSON file and deserializes it into a data structure.
/// This function will error if the file cannot be read or deserialized into the aforementioned data structure.
pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, Error> {
    let val = _read_json_inner(path)?;
    from_value(val).map_err(Error::json)
}

/// Writes an object to a JSON file.
/// This function will error if the file cannot be created or written to.
pub fn write_json<T: Serialize, P: AsRef<Path>>(path: P, value: T) -> Result<(), Error> {
    write_json_with_indent(path, value, 4)?;
    Ok(())
}

/// Writes an object to a JSON file with a custom indentation.
pub fn write_json_with_indent<T: Serialize, P: AsRef<Path>>(path: P, value: T, indent: usize) -> Result<(), Error> {
    _create_parent_dir(&path)?;

    let indent = " ".repeat(indent);
    let indent = indent.as_bytes();

    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(indent);
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser).map_err(Error::json)?;
    write(path, buf).map_err(Error::io)?;

    Ok(())
}
